use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use futures::executor::block_on;
use polywrap_core::invoke::{InvokeArgs, InvokeOptions, Invoker};
use polywrap_core::uri::Uri;
use wasmtime::*;

use crate::error::WrapperError;
use crate::memory::{read_from_memory, write_to_memory};
use crate::utils::index_of_array;

pub struct WasmInstance {
    instance: Instance,
    pub shared_state: Arc<Mutex<State>>,
    store: Store<u32>,
    pub module: Module,
}

pub enum WasmModule {
    Bytes(Vec<u8>),
    Wat(String),
    Path(String),
}

#[derive(Default)]
pub struct InvokeState {
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
}

#[derive(Default)]
pub struct State {
    pub method: Vec<u8>,
    pub args: Vec<u8>,
    pub invoke: InvokeState,
    pub subinvoke: InvokeState,
}

impl State {
    pub fn new(method: &str, args: Vec<u8>) -> Self {
        Self {
            method: method.as_bytes().to_vec(),
            args,
            invoke: InvokeState::default(),
            subinvoke: InvokeState::default(),
        }
    }
}

impl WasmInstance {
    pub fn new(
        wasm_module: &WasmModule,
        shared_state: Arc<Mutex<State>>,
        abort: Arc<dyn Fn(String) + Send + Sync>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Self, WrapperError> {
        let mut config = Config::new();
        config.async_support(true);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let engine =
            Engine::new(&config).map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;
        let mut linker = wasmtime::Linker::new(&engine);

        let mut store = Store::new(&engine, 4);
        let module_result = match wasm_module {
            WasmModule::Bytes(ref bytes) => Module::new(&engine, bytes),
            WasmModule::Wat(ref wat) => Module::new(&engine, wat),
            WasmModule::Path(ref path) => Module::from_file(&engine, path),
        };

        let module = module_result.map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;
        let module_bytes = module
            .serialize()
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let memory = Rc::new(RefCell::new(WasmInstance::create_memory(
            module_bytes.as_ref(),
            &mut store,
        )?));

        Self::create_imports(
            &mut linker,
            Arc::clone(&shared_state),
            abort,
            memory,
            invoker,
        )?;

        let instance = rt
            .block_on(linker.instantiate_async(store.as_context_mut(), &module))
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        Ok(Self {
            module,
            shared_state,
            instance,
            store,
        })
    }

    fn create_imports(
        linker: &mut Linker<u32>,
        shared_state: Arc<Mutex<State>>,
        abort: Arc<dyn Fn(String) + Send + Sync>,
        memory: Rc<RefCell<Memory>>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<(), WrapperError> {
        let arc_shared_state = Arc::clone(&shared_state);
        let arc_memory = Arc::new(Mutex::new(memory.borrow_mut().to_owned()));
        let mem = arc_memory.clone();
        let abort_clone = Arc::clone(&abort);

        linker
            .func_wrap(
                "wrap",
                "__wrap_invoke_args",
                move |mut caller: Caller<'_, u32>, method_ptr: u32, args_ptr: u32| {
                    let state_guard = arc_shared_state.lock().unwrap();

                    if state_guard.method.is_empty() {
                        abort_clone("__wrap_invoke_args: method is not set".to_string());
                    }

                    if state_guard.args.is_empty() {
                        abort_clone("__wrap_invoke_args: args is not set".to_string());
                    }

                    write_to_memory(
                        arc_memory.clone(),
                        caller.as_context_mut(),
                        method_ptr.try_into().unwrap(),
                        state_guard.method.as_ref(),
                    );

                    write_to_memory(
                        arc_memory.clone(),
                        caller.as_context_mut(),
                        args_ptr.try_into().unwrap(),
                        state_guard.args.as_ref(),
                    );

                    Ok(())
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_invoke_result",
                move |caller: Caller<'_, u32>, ptr: u32, len: u32| {
                    let mut state_ref = arc_shared_state.lock().unwrap();
                    let memory_data = read_from_memory(
                        arc_memory.clone(),
                        caller.as_context(),
                        ptr.try_into().unwrap(),
                        len.try_into().unwrap(),
                    );
                    state_ref.invoke.result = Some(memory_data);
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_invoke_error",
                move |caller: Caller<'_, u32>, ptr: u32, len: u32| {
                    dbg!("__wrap_invoke_error");
                    let mut state_ref = arc_shared_state.lock().unwrap();
                    let memory_data = read_from_memory(
                        arc_memory.clone(),
                        caller.as_context(),
                        ptr.try_into().unwrap(),
                        len.try_into().unwrap(),
                    );
                    state_ref.invoke.error = Some(String::from_utf8(memory_data).unwrap());
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let abort_clone = Arc::clone(&abort);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_abort",
                move |caller: Caller<'_, u32>,
                      msg_ptr: u32,
                      msg_len: u32,
                      file_ptr: u32,
                      file_len: u32,
                      line: u32,
                      column: u32| {
                    let msg = read_from_memory(
                        arc_memory.clone(),
                        caller.as_context(),
                        msg_ptr.try_into().unwrap(),
                        msg_len.try_into().unwrap(),
                    );
                    let file = read_from_memory(
                        arc_memory.clone(),
                        caller.as_context(),
                        file_ptr.try_into().unwrap(),
                        file_len.try_into().unwrap(),
                    );

                    let msg_str = String::from_utf8(msg).unwrap();
                    let file_str = String::from_utf8(file).unwrap();

                    abort_clone(format!(
                        "__wrap_abort: {msg}\nFile: {file}\nLocation: [{line},{column}]",
                        msg = msg_str,
                        file = file_str,
                        line = line,
                        column = column
                    ));
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_subinvoke",
                move |caller: Caller<'_, u32>,
                      uri_ptr: u32,
                      uri_len: u32,
                      method_ptr: u32,
                      method_len: u32,
                      args_ptr: u32,
                      args_len: u32| {
                    let mut state_ref = arc_shared_state.lock().unwrap();

                    let uri_bytes = read_from_memory(
                        arc_memory.clone(),
                        caller.as_context(),
                        uri_ptr.try_into().unwrap(),
                        uri_len.try_into().unwrap(),
                    );
                    let method_bytes = read_from_memory(
                        arc_memory.clone(),
                        caller.as_context(),
                        method_ptr.try_into().unwrap(),
                        method_len.try_into().unwrap(),
                    );
                    let args_bytes = read_from_memory(
                        arc_memory.clone(),
                        caller.as_context(),
                        args_ptr.try_into().unwrap(),
                        args_len.try_into().unwrap(),
                    );

                    let uri = Uri::from_string(&String::from_utf8(uri_bytes).unwrap()).unwrap();
                    let method = String::from_utf8(method_bytes).unwrap();
                    let invoke_args = InvokeArgs::UIntArray(args_bytes);

                    let invoker_opts = InvokeOptions {
                        uri: &uri,
                        method: &method,
                        args: Some(&invoke_args),
                        env: None,
                        resolution_context: None,
                    };

                    let result = block_on(invoker.invoke(&invoker_opts));

                    match result {
                        Ok(res) => {
                            state_ref.subinvoke.result = Some(res);
                            1
                        }
                        Err(err) => {
                            state_ref.subinvoke.error = Some(err.to_string());
                            0
                        }
                    }
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let abort_clone = Arc::clone(&abort);

        linker
            .func_wrap("wrap", "__wrap_subinvoke_result_len", move || {
                let state_ref = arc_shared_state.lock().unwrap();

                match &state_ref.subinvoke.result {
                    Some(res) => res.len() as u32,
                    None => {
                        abort_clone(
                            "__wrap_subinvoke_result_len: subinvoke.result is not set".to_string(),
                        );
                        0
                    }
                }
            })
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let abort_clone = Arc::clone(&abort);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_subinvoke_result",
                move |mut caller: Caller<'_, u32>, ptr: u32| {
                    let state_ref = arc_shared_state.lock().unwrap();

                    match &state_ref.subinvoke.result {
                        Some(res) => write_to_memory(
                            arc_memory.clone(),
                            caller.as_context_mut(),
                            ptr.try_into().unwrap(),
                            res,
                        ),
                        None => {
                            abort_clone(
                                "__wrap_subinvoke_result: subinvoke.result is not set".to_string(),
                            );
                        }
                    }
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let abort_clone = Arc::clone(&abort);

        linker
            .func_wrap("wrap", "__wrap_subinvoke_error_len", move || {
                let state_ref = arc_shared_state.lock().unwrap();

                match &state_ref.subinvoke.error {
                    Some(res) => res.len() as u32,
                    None => {
                        abort_clone(
                            "__wrap_subinvoke_error_len: subinvoke.error is not set".to_string(),
                        );
                        0
                    }
                }
            })
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let abort_clone = Arc::clone(&abort);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_subinvoke_error",
                move |mut caller: Caller<'_, u32>, ptr: u32| {
                    let state_ref = arc_shared_state.lock().unwrap();

                    match &state_ref.subinvoke.error {
                        Some(res) => write_to_memory(
                            arc_memory.clone(),
                            caller.as_context_mut(),
                            ptr.try_into().unwrap(),
                            res.as_bytes(),
                        ),
                        None => {
                            abort_clone(
                                "__wrap_subinvoke_error: subinvoke.error is not set".to_string(),
                            );
                        }
                    }
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        linker
            .define("env", "memory", memory.borrow_mut().to_owned())
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        Ok(())
    }

    pub fn call_export(
        &mut self,
        name: &str,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<(), WrapperError> {
        let export = self.instance.get_export(self.store.as_context_mut(), name);

        if export.is_none() {
            return Err(WrapperError::WasmRuntimeError(format!(
                "Export {} not found",
                name
            )));
        }

        match export.unwrap() {
            Extern::Func(func) => {
                block_on(func.call_async(self.store.as_context_mut(), params, results))
                    .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

                Ok(())
            }
            _ => panic!("Export is not a function"),
        }
    }

    fn create_memory(module_bytes: &[u8], store: &mut Store<u32>) -> Result<Memory, WrapperError> {
        const ENV_MEMORY_IMPORTS_SIGNATURE: [u8; 11] = [
            0x65, 0x6e, 0x76, 0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x02,
        ];

        let sig_idx = index_of_array(module_bytes, &ENV_MEMORY_IMPORTS_SIGNATURE);

        if sig_idx.is_none() {
            return Err(WrapperError::ModuleReadError(
                r#"Unable to find Wasm memory import section.
            Modules must import memory from the "env" module's
            "memory" field like so:
            (import "env" "memory" (memory (;0;) #))"#
                    .to_string(),
            ));
        }

        let memory_initial_limits =
            module_bytes[sig_idx.unwrap() + ENV_MEMORY_IMPORTS_SIGNATURE.len() + 1];
        let memory_type = MemoryType::new(memory_initial_limits.into(), Option::None);

        Memory::new(store.as_context_mut(), memory_type)
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))
    }
}
