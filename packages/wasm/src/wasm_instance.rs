use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use futures::executor::block_on;
use polywrap_core::invoke::{InvokeOptions, Invoker, InvokerOptions};
use polywrap_core::uri::uri::Uri;
use wasmtime::*;

use crate::error::WrapperError;
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

impl WasmInstance {
    pub fn new(
        wasm_module: &WasmModule,
        shared_state: Arc<Mutex<State>>,
        abort: Arc<dyn Fn(String) + Send + Sync>,
        invoker: Arc<Mutex<dyn Invoker>>,
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
        invoker: Arc<Mutex<dyn Invoker>>,
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
                    let memory = arc_memory.lock().unwrap();

                    if state_guard.method.is_empty() {
                        abort_clone("__wrap_invoke_args: method is not set".to_string());
                    }

                    if state_guard.args.is_empty() {
                        abort_clone("__wrap_invoke_args: args is not set".to_string());
                    }

                    let mem_data = memory.data_mut(caller.as_context_mut());
                    mem_data[method_ptr as usize..method_ptr as usize + state_guard.method.len()]
                        .copy_from_slice(&state_guard.method);

                    mem_data[args_ptr as usize..args_ptr as usize + state_guard.args.len()]
                        .copy_from_slice(&state_guard.args);
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_invoke_result",
                move |mut caller: Caller<'_, u32>, ptr: u32, len: u32| {
                    let mut state_ref = arc_shared_state.lock().unwrap();
                    let memory_guard = arc_memory.lock().unwrap();

                    let mem_data = memory_guard.data_mut(caller.as_context_mut());
                    let res = mem_data[ptr as usize..ptr as usize + len as usize].to_vec();
                    state_ref.invoke.result = Some(res);
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let arc_shared_state = Arc::clone(&shared_state);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_invoke_error",
                move |mut caller: Caller<'_, u32>, ptr: u32, len: u32| {
                    dbg!("__wrap_invoke_error");
                    let mut state_ref = arc_shared_state.lock().unwrap();
                    let memory_guard = arc_memory.lock().unwrap();

                    let mem_data = memory_guard.data_mut(caller.as_context_mut());
                    let res = mem_data[ptr as usize..ptr as usize + len as usize].to_vec();
                    state_ref.invoke.error = Some(String::from_utf8(res).unwrap());
                },
            )
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let abort_clone = Arc::clone(&abort);
        let arc_memory = Arc::clone(&mem);

        linker
            .func_wrap(
                "wrap",
                "__wrap_abort",
                move |mut caller: Caller<'_, u32>,
                      msg_ptr: u32,
                      msg_len: u32,
                      file_ptr: u32,
                      file_len: u32,
                      line: u32,
                      column: u32| {
                    let memory = arc_memory.lock().unwrap();

                    let mem_data = memory.data_mut(caller.as_context_mut());
                    let msg =
                        mem_data[msg_ptr as usize..msg_ptr as usize + msg_len as usize].to_vec();
                    let file =
                        mem_data[file_ptr as usize..file_ptr as usize + file_len as usize].to_vec();
                    let msg = String::from_utf8(msg).unwrap();
                    let file = String::from_utf8(file).unwrap();

                    abort_clone(format!(
                        "__wrap_abort: {msg}\nFile: {file}\nLocation: [{line},{column}]",
                        msg = msg,
                        file = file,
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
                "__wrap_async",
                move |caller: Caller<'_, u32>,
                      uri_ptr: u32,
                      uri_len: u32,
                      method_ptr: u32,
                      method_len: u32,
                      args_ptr: u32,
                      args_len: u32| {
                    let memory = arc_memory.lock().unwrap();
                    let mut state_ref = arc_shared_state.lock().unwrap();

                    let memory_data = memory.data(caller.as_context());

                    let uri_bytes =
                        memory_data[uri_ptr as usize..uri_ptr as usize + uri_len as usize].to_vec();
                    let uri = Uri::from_string(&String::from_utf8(uri_bytes).unwrap()).unwrap();

                    let method_bytes = memory_data
                        [method_ptr as usize..method_ptr as usize + method_len as usize]
                        .to_vec();
                    let method = String::from_utf8(method_bytes).unwrap();

                    let args_bytes = memory_data
                        [args_ptr as usize..args_ptr as usize + args_len as usize]
                        .to_vec();

                    let invoker_opts = InvokerOptions {
                        invoke_options: InvokeOptions {
                            uri: &uri,
                            method: &method,
                            args: Some(&args_bytes),
                            env: None,
                            resolution_context: None,
                        },
                        encode_result: true,
                    };

                    let result = block_on(invoker.lock().unwrap().invoke(&invoker_opts));

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
