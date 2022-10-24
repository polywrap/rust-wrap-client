use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use futures::executor::block_on;
use polywrap_core::{
    invoke::{InvokeArgs, InvokeOptions, Invoker},
    uri::Uri,
};
use wasmtime::{AsContext, AsContextMut, Caller, Linker, Memory, StoreContext, StoreContextMut};

use crate::{error::WrapperError, wasm_runtime::instance::State};

fn write_to_memory(
    memory: Arc<Mutex<Memory>>,
    store_ctx: StoreContextMut<'_, u32>,
    offset: usize,
    data: &[u8],
) {
    let memory_guard = memory.lock().unwrap();
    memory_guard.data_mut(store_ctx)[offset..offset + data.len()].copy_from_slice(data);
}

fn read_from_memory(
    memory: Arc<Mutex<Memory>>,
    store_ctx: StoreContext<'_, u32>,
    offset: usize,
    length: usize,
) -> Vec<u8> {
    let memory_guard = memory.lock().unwrap();
    memory_guard.data(store_ctx)[offset..offset + length].to_vec()
}

pub fn create_imports(
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
