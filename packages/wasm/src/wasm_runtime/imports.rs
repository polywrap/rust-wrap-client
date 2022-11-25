use std::sync::{Arc, Mutex};

use polywrap_core::{
    invoke::{InvokeArgs},
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

                    let result = state.invoker.invoke(&uri, &method, Some(&invoke_args), None, None).await;
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
        .func_wrap8_async(
            "wrap",
            "__wrap_subinvokeImplementation",
            move |mut caller: Caller<'_, State>,
                  interface_ptr: u32,
                  interface_len: u32,
                  impl_uri_ptr: u32,
                  impl_uri_len: u32,
                  method_ptr: u32,
                  method_len: u32,
                  args_ptr: u32,
                  args_len: u32| {
                let memory = memory.lock().unwrap();
                let async_memory = Arc::new(tokio::sync::Mutex::new(*memory));

                Box::new(async move {
                    let memory = async_memory.lock().await;
                    let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                    let interface_uri_bytes = read_from_memory(memory_buffer, interface_ptr as usize, interface_len as usize);
                    let interface = String::from_utf8(interface_uri_bytes).unwrap();

                    let impl_uri_bytes = read_from_memory(memory_buffer, impl_uri_ptr as usize, impl_uri_len as usize);
                    let method_bytes = read_from_memory(memory_buffer, method_ptr as usize, method_len as usize);
                    let args_bytes = read_from_memory(memory_buffer, args_ptr as usize, args_len as usize);

                    let uri = Uri::from_string(&String::from_utf8(impl_uri_bytes).unwrap()).unwrap();
                    let method = String::from_utf8(method_bytes).unwrap();
                    let invoke_args = InvokeArgs::UIntArray(args_bytes);

                    let result = state.invoker.invoke(&uri, &method, Some(&invoke_args), None, None).await;
                    match result {
                        Ok(res) => {
                            state.subinvoke.result = Some(res);
                            1
                        }
                        Err(err) => {
                            let error = format!("interface implementation subinvoke failed for uri: {} with error: {}", interface, err);
                            state.subinvoke.error = Some(error);
                            0
                        }
                    }
               })
            }
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    linker
        .func_wrap(
            "wrap",
            "__wrap_subinvokeImplementation_result_len",
            move |caller: Caller<'_, State>| {
                let state = caller.data();
                match &state.subinvoke_implementation {
                    Some(subinvoke_impl_state) => {
                        if let Some(result) = &subinvoke_impl_state.result {
                            return result.len() as u32;
                        }
                        (state.abort)(
                            "__wrap_subinvokeImplementation_result_len: subinvoke_implementation.result is not set".to_string(),
                        );
                        0
                    }
                    None => {
                        (state.abort)(
                            "__wrap_subinvokeImplementation_result_len: subinvoke_implementation is not set".to_string(),
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
            "__wrap_subinvokeImplementation_result",
            move |mut caller: Caller<'_, State>, ptr: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                match &state.subinvoke_implementation {
                    Some(subinvoke_impl_state) => {
                        if let Some(result) = &subinvoke_impl_state.result {
                            write_to_memory(memory_buffer, ptr as usize, result);
                        }
                        (state.abort)(
                            "__wrap_subinvokeImplementation_result: subinvoke_implementation.result is not set".to_string(),
                        );
                    },
                    None => {
                        (state.abort)(
                            "__wrap_subinvokeImplementation_result: subinvoke_implementation is not set".to_string(),
                        );
                    }
                }
            }
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    linker
        .func_wrap(
            "wrap",
            "__wrap_subinvokeImplementation_error_len",
            move |caller: Caller<'_, State>| {
                let state = caller.data();
                match &state.subinvoke_implementation {
                    Some(subinvoke_impl_state) => {
                        if let Some(error) = &subinvoke_impl_state.error {
                            return error.len() as u32;
                        }
                        (state.abort)(
                            "__wrap_subinvokeImplementation_error_len: subinvoke_implementation.error is not set".to_string(),
                        );
                        0
                    }
                    None => {
                        (state.abort)(
                            "__wrap_subinvokeImplementation_error_len: subinvoke_implementation is not set".to_string(),
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
            "__wrap_subinvokeImplementation_error",
            move |mut caller: Caller<'_, State>, ptr: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                match &state.subinvoke_implementation {
                    Some(subinvoke_impl_state) => {
                        if let Some(error) = &subinvoke_impl_state.error {
                            write_to_memory(memory_buffer, ptr as usize, error.as_bytes());
                        }
                        (state.abort)(
                            "__wrap_subinvokeImplementation_error: subinvoke_implementation.error is not set".to_string(),
                        );
                    },
                    None => {
                        (state.abort)(
                            "__wrap_subinvokeImplementation_error: subinvoke_implementation is not set".to_string(),
                        );
                    }
                }
            }
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap(
            "wrap",
            "__wrap_getImplementations",
            move |mut caller: Caller<'_, State>, ptr: u32, len: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                let uri_bytes = read_from_memory(memory_buffer, ptr as usize, len as usize);
                let uri = String::from_utf8(uri_bytes).unwrap();
                let result = state.invoker.get_implementations(Uri::new(uri.as_str()));

                if result.is_err() {
                    let result = result.as_ref().err().unwrap();
                    (state.abort)(result.to_string());
                    return 0;
                }

                let implementations = &result.unwrap();
                let encoded_implementations = rmp_serde::encode::to_vec(implementations);                
                state.get_implementations_result = Some(encoded_implementations.unwrap());

                if !state.get_implementations_result.as_ref().unwrap().is_empty() {
                    return 1;
                }

                0
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    linker
        .func_wrap(
            "wrap", 
            "__wrap_getImplementations_result_len", 
            move |caller: Caller<'_, State>| {
                let state = caller.data();

                match &state.get_implementations_result {
                    Some(result) => result.len() as u32,
                    None => {
                        (state.abort)("__wrap_getImplementations_result_len: result is not set".to_string());
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
            "__wrap_getImplementations_result", 
            move |mut caller: Caller<'_, State>, ptr: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                match &state.get_implementations_result {
                    Some(result) => {
                        write_to_memory(memory_buffer, ptr as usize, result);
                    },
                    None => {
                        (state.abort)("__wrap_getImplementations_result: result is not set".to_string());
                    }
                };
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .func_wrap(
            "wrap",
            "__wrap_load_env",
            move |mut caller: Caller<'_, State>, ptr: u32| {
                let memory = memory.lock().unwrap();
                let (memory_buffer, state) = memory.data_and_store_mut(caller.as_context_mut());

                write_to_memory(memory_buffer, ptr as usize, &state.env);
                Ok(())
            },
        )
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    let memory = Arc::clone(&arc_memory);
    linker
        .define("env", "memory", *memory.lock().unwrap())
        .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

    Ok(())
}
