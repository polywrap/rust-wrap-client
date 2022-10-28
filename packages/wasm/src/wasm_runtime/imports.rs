use std::sync::{Arc, Mutex};

use polywrap_core::{
    invoke::{InvokeArgs, InvokeOptions},
    uri::Uri,
};
use wasmtime::{AsContextMut, Caller, Linker, Memory};

use crate::{error::WrapperError, wasm_runtime::instance::State};

fn read_from_memory(buffer: &mut [u8], ptr: usize, len: usize) -> Vec<u8> {
    buffer[ptr..(ptr + len)].to_vec()
}

fn write_to_memory(buffer: &mut [u8], ptr: usize, data: &[u8]) {
    buffer[ptr..(ptr + data.len())].copy_from_slice(data);
}

pub fn create_imports(
    linker: &mut Linker<State>,
    arc_memory: Arc<Mutex<Memory>>,
) -> Result<(), WrapperError> {
    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap(
            "wrap",
            "__wrap_invoke_args",
            move |mut caller: Caller<'_, State>, method_ptr: u32, args_ptr: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                if state.method.is_empty() {
                    (state.abort)("__wrap_invoke_args: method is not set".to_string());
                }

                if state.args.is_empty() {
                    (state.abort)("__wrap_invoke_args: args is not set".to_string());
                }

                write_to_memory(memory_buffer, method_ptr as usize, &state.method);
                write_to_memory(memory_buffer, args_ptr as usize, &state.args);

                Ok(())
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);

    linker
        .func_wrap(
            "wrap",
            "__wrap_invoke_result",
            move |mut caller: Caller<'_, State>, ptr: u32, len: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());
                let memory_data = read_from_memory(memory_buffer, ptr as usize, len as usize);
                state.invoke.result = Some(memory_data);
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap(
            "wrap",
            "__wrap_invoke_error",
            move |mut caller: Caller<'_, State>, ptr: u32, len: u32| {
                dbg!("__wrap_invoke_error");
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());
                let memory_data = read_from_memory(memory_buffer, ptr as usize, len as usize);

                state.invoke.error = Some(String::from_utf8(memory_data).unwrap());
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap(
            "wrap",
            "__wrap_abort",
            move |mut caller: Caller<'_, State>,
                  msg_ptr: u32,
                  msg_len: u32,
                  file_ptr: u32,
                  file_len: u32,
                  line: u32,
                  column: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());
                let msg = read_from_memory(memory_buffer, msg_ptr as usize, msg_len as usize);
                let file = read_from_memory(memory_buffer, file_ptr as usize, file_len as usize);

                let msg_str = String::from_utf8(msg).unwrap();
                let file_str = String::from_utf8(file).unwrap();

                (state.abort)(format!(
                    "__wrap_abort: {msg}\nFile: {file}\nLocation: [{line},{column}]",
                    msg = msg_str,
                    file = file_str,
                    line = line,
                    column = column
                ));
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap6_async(
            "wrap",
            "__wrap_subinvoke",
            move |mut caller: Caller<'_, State>,
                  uri_ptr: u32,
                  uri_len: u32,
                  method_ptr: u32,
                  method_len: u32,
                  args_ptr: u32,
                  args_len: u32| {
                let memory = memory.lock().unwrap();
                let async_memory = Arc::new(tokio::sync::Mutex::new(*memory));

                Box::new(async move {
                    let memory = async_memory.lock().await;
                    let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());
                    let uri_bytes =
                        read_from_memory(memory_buffer, uri_ptr as usize, uri_len as usize);
                    let method_bytes =
                        read_from_memory(memory_buffer, method_ptr as usize, method_len as usize);
                    let args_bytes = read_from_memory(memory_buffer, args_ptr as usize, args_len as usize);

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

                    let result = state.invoker.invoke(&invoker_opts).await;
                    match result {
                        Ok(res) => {
                            state.subinvoke.result = Some(res);
                            1
                        }
                        Err(err) => {
                            state.subinvoke.error = Some(err.to_string());
                            0
                        }
                    }
                })
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    linker
        .func_wrap(
            "wrap",
            "__wrap_subinvoke_result_len",
            move |caller: Caller<'_, State>| {
                let state = caller.data();

                match &state.subinvoke.result {
                    Some(res) => res.len() as u32,
                    None => {
                        (state.abort)(
                            "__wrap_subinvoke_result_len: subinvoke.result is not set".to_string(),
                        );
                        0
                    }
                }
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap(
            "wrap",
            "__wrap_subinvoke_result",
            move |mut caller: Caller<'_, State>, ptr: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                match &state.subinvoke.result {
                    Some(res) => {
                        write_to_memory(memory_buffer, ptr as usize, res);
                    }
                    None => {
                        (state.abort)(
                            "__wrap_subinvoke_result: subinvoke.result is not set".to_string(),
                        );
                    }
                }
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    linker
        .func_wrap(
            "wrap",
            "__wrap_subinvoke_error_len",
            move |caller: Caller<'_, State>| {
                let state = caller.data();

                match &state.subinvoke.error {
                    Some(res) => res.len() as u32,
                    None => {
                        (state.abort)(
                            "__wrap_subinvoke_error_len: subinvoke.error is not set".to_string(),
                        );
                        0
                    }
                }
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap(
            "wrap",
            "__wrap_subinvoke_error",
            move |mut caller: Caller<'_, State>, ptr: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                match &state.subinvoke.error {
                    Some(res) => write_to_memory(memory_buffer, ptr as usize, res.as_bytes()),
                    None => {
                        (state.abort)(
                            "__wrap_subinvoke_error: subinvoke.error is not set".to_string(),
                        );
                    }
                }
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .define("env", "memory", *memory.lock().unwrap())
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    Ok(())
}
