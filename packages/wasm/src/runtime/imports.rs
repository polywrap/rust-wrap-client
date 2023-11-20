use std::sync::{Arc, Mutex};

use polywrap_msgpack_serde::to_vec;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, FunctionType, Imports, Memory, Store, Type,
    Value, RuntimeError,
};

use crate::runtime::instance::SubinvokeImplementationState;

use super::instance::State;

pub fn create_imports(memory: Memory, store: &mut Store, state: Arc<Mutex<State>>) -> Imports {
    let context = FunctionEnv::new(store, state);

    let invoke_args = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let method_ptr = values[0].unwrap_i32() as u64;
        let args_ptr = values[1].unwrap_i32() as u64;

        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        if mutable_state.method.is_empty() {
            return Err(RuntimeError::new(
                "__wrap_invoke_args: method is not set"
            ));
        }

        if mutable_state.args.is_empty() {
            return Err(RuntimeError::new(
                "__wrap_invoke_args: args is not set"
            ));
        }

        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);

        memory_view
            .write(method_ptr, &mutable_state.method)
            .unwrap();
        memory_view
            .write(args_ptr, &mutable_state.args)
            .unwrap();
        Ok(vec![])
    };

    let invoke_args_signature = FunctionType::new(vec![Type::I32, Type::I32], vec![]);

    let wrap_invoke_args =
        Function::new_with_env(store, &context, invoke_args_signature, invoke_args);

    let invoke_result = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mut mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);
        let offset = values[0].unwrap_i32() as u64;
        let length = values[1].unwrap_i32() as usize;

        let mut buffer: Vec<u8> = Vec::with_capacity(length);
        memory_view
            .read(offset, &mut buffer)
            .map_err(|e| RuntimeError::new(e.to_string()))?;
        mutable_state.invoke.result = Some(buffer);
        Ok(vec![])
    };

    let invoke_result_signature = FunctionType::new(vec![Type::I32, Type::I32], vec![]);

    let wrap_invoke_result =
        Function::new_with_env(store, &context, invoke_result_signature, invoke_result);

    let invoke_result_error_signature = FunctionType::new(vec![Type::I32, Type::I32], vec![]);

    let invoke_error = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mut mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);
        let offset = values[0].unwrap_i32() as u64;
        let length = values[1].unwrap_i32() as usize;

        let mut buffer: Vec<u8> = Vec::with_capacity(length);
        memory_view
            .read(offset, &mut buffer)
            .map_err(|e| RuntimeError::new(e.to_string()))?;

        let invoke_error = String::from_utf8(buffer)
            .map_err(|e|RuntimeError::new(format!("__wrap_invoke_error: {}", e.to_string())))?;

        mutable_state.invoke.error = Some(invoke_error);
        Ok(vec![])
    };

    let wrap_invoke_error =
        Function::new_with_env(store, &context, invoke_result_error_signature, invoke_error);

    let invoke_abort_signature = FunctionType::new(
        vec![
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
        ],
        vec![],
    );

    let abort = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let msg_offset = values[0].unwrap_i32() as u64;
        let msg_length = values[1].unwrap_i32() as usize;
        let file_offset = values[2].unwrap_i32() as u64;
        let file_length = values[3].unwrap_i32() as usize;
        let line = values[4].unwrap_i32() as u64;
        let column = values[5].unwrap_i32() as usize;

        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();
        let memory = state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);

        let mut msg_buffer: Vec<u8> = Vec::with_capacity(msg_length);
        let mut file_buffer: Vec<u8> = Vec::with_capacity(file_length);

        memory_view
            .read(msg_offset, &mut msg_buffer)
            .unwrap();
        memory_view
            .read(file_offset, &mut file_buffer)
            .unwrap();

        let msg = String::from_utf8(msg_buffer).unwrap();
        let file = String::from_utf8(file_buffer).unwrap();

        Err(RuntimeError::new(format!(
            "__wrap_abort: {msg}\nFile: {file}\nLocation: [{line},{column}]"
        )))
    };

    let wrap_abort = Function::new_with_env(store, &context, invoke_abort_signature, abort);

    let subinvoke_signature = FunctionType::new(
        vec![
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
        ],
        vec![Type::I32],
    );

    let subinvoke = move |context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let uri_ptr = values[0].unwrap_i32() as u64;
        let uri_len = values[1].unwrap_i32() as usize;
        let method_ptr = values[2].unwrap_i32() as u64;
        let method_len = values[3].unwrap_i32() as usize;
        let args_ptr = values[4].unwrap_i32() as u64;
        let args_len = values[5].unwrap_i32() as usize;

        let async_context = Arc::new(Mutex::new(context));
        let mut context = async_context.lock().unwrap();
        let mutable_context = context.as_mut();
        let mut state = mutable_context.data().lock().unwrap();

        let memory = state.memory.as_ref().unwrap();
        let mut uri_buffer: Vec<u8> = Vec::with_capacity(uri_len);
        let mut method_buffer: Vec<u8> = Vec::with_capacity(method_len);
        let mut args_buffer: Vec<u8> = Vec::with_capacity(args_len);

        memory
            .view(&mutable_context)
            .read(uri_ptr, &mut uri_buffer)
            .unwrap();
        memory
            .view(&mutable_context)
            .read(method_ptr, &mut method_buffer)
            .unwrap();
        memory
            .view(&mutable_context)
            .read(args_ptr, &mut args_buffer)
            .unwrap();

        let uri = String::from_utf8(uri_buffer).unwrap();
        let method = String::from_utf8(method_buffer).unwrap();
        let mut _decoded_env = serde_json::Value::Null;

        let uri = uri.clone().try_into()
            .map_err(|_| RuntimeError::new(format!("__wrap_subinvoke: invalid uri: {}", uri)))?;

        let result =
            state
                .invoker
                .clone()
                .invoke_raw(&uri, &method, Some(&args_buffer), None, None);

        match result {
            Ok(res) => {
                state.subinvoke.result = Some(res);
                Ok(vec![Value::I32(1)])
            }
            Err(err) => {
                state.subinvoke.error = Some(err.to_string());
                Ok(vec![Value::I32(0)])
            }
        }
    };

    let wrap_subinvoke = Function::new_with_env(store, &context, subinvoke_signature, subinvoke);

    let subinvoke_result_len_signature = FunctionType::new(vec![], vec![Type::I32]);

    let subinvoke_result_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>,
                                     _: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        let result = mutable_state.subinvoke.result.as_ref()
            .ok_or(RuntimeError::new(
                "__wrap_subinvoke_result_len: subinvoke.result is not set",
            ))?;
        
        Ok(vec![Value::I32(result.len() as i32)])
    };

    let wrap_subinvoke_result_len = Function::new_with_env(
        store,
        &context,
        subinvoke_result_len_signature,
        subinvoke_result_len,
    );

    let subinvoke_result_signature = FunctionType::new(vec![Type::I32], vec![]);

    let subinvoke_result = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>,
                                 values: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();

        let pointer = values[0].unwrap_i32() as u64;

        let result = mutable_state.subinvoke.result.as_ref()
            .ok_or(RuntimeError::new(
                "__wrap_subinvoke_result: subinvoke.result is not set",
            ))?;

        memory
            .view(&mutable_context)
            .write(pointer, result)
            .map(|_| vec![])
            .map_err(|e| RuntimeError::new(e.to_string()))
    };

    let wrap_subinvoke_result = Function::new_with_env(
        store,
        &context,
        subinvoke_result_signature,
        subinvoke_result,
    );

    let subinvoke_error_len_signature = FunctionType::new(vec![], vec![Type::I32]);

    let subinvoke_error_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        let subinvoke_error = mutable_state.subinvoke.error.as_ref()
            .ok_or(RuntimeError::new(
                "__wrap_subinvoke_error_len: subinvoke.error is not set",
            ))?;
        
        Ok(vec![Value::I32(subinvoke_error.len() as i32)])
    };

    let wrap_subinvoke_error_len = Function::new_with_env(
        store,
        &context,
        subinvoke_error_len_signature,
        subinvoke_error_len,
    );

    let subinvoke_error_signature = FunctionType::new(vec![Type::I32], vec![]);

    let subinvoke_error = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>,
                                values: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();

        let pointer = values[0].unwrap_i32() as u64;
        
        let subinvoke_error = mutable_state.subinvoke.error.as_ref()
            .ok_or(RuntimeError::new(
                "__wrap_subinvoke_error: subinvoke.error is not set",
            ))?;

        memory
            .view(&mutable_context)
            .write(pointer, subinvoke_error.as_bytes())
            .map(|_| vec![])
            .map_err(|e| RuntimeError::new(e.to_string()))
    };

    let wrap_subinvoke_error =
        Function::new_with_env(store, &context, subinvoke_error_signature, subinvoke_error);

    let subinvoke_implementation_signature = FunctionType::new(
        vec![
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
            Type::I32,
        ],
        vec![Type::I32],
    );

    let subinvoke_implementation = move |context: FunctionEnvMut<Arc<Mutex<State>>>,
                                         values: &[Value]| {
        let interface_ptr = values[0].unwrap_i32() as u64;
        let interface_len = values[1].unwrap_i32() as usize;
        let impl_uri_ptr = values[2].unwrap_i32() as u64;
        let impl_uri_len = values[3].unwrap_i32() as usize;
        let method_ptr = values[4].unwrap_i32() as u64;
        let method_len = values[5].unwrap_i32() as usize;
        let args_ptr = values[6].unwrap_i32() as u64;
        let args_len = values[7].unwrap_i32() as usize;

        let async_context = Arc::new(Mutex::new(context));
        let mut context = async_context.lock().unwrap();
        let mutable_context = context.as_mut();
        let mut state = mutable_context.data().lock().unwrap();

        let mut interface_buffer = Vec::with_capacity(interface_len);
        let mut impl_uri_buffer = Vec::with_capacity(impl_uri_len);
        let mut method_buffer = Vec::with_capacity(method_len);
        let mut args_buffer = Vec::with_capacity(args_len);

        let memory = state.memory.as_ref().unwrap();
        memory
            .view(&mutable_context)
            .read(interface_ptr, &mut interface_buffer)
            .unwrap();
        memory
            .view(&mutable_context)
            .read(impl_uri_ptr, &mut impl_uri_buffer)
            .unwrap();
        memory
            .view(&mutable_context)
            .read(method_ptr, &mut method_buffer)
            .unwrap();
        memory
            .view(&mutable_context)
            .read(args_ptr, &mut args_buffer)
            .unwrap();

        let interface = String::from_utf8(interface_buffer).unwrap();
        let uri: String = String::from_utf8(impl_uri_buffer).unwrap();
        let method = String::from_utf8(method_buffer).unwrap();
        let mut _decoded_env = serde_json::Value::Null;

        let uri = uri.clone().try_into()
            .map_err(|_| RuntimeError::new(format!("__wrap_subinvokeImplementation: invalid uri: {}", uri)))?;

        let result = state.invoker.clone().invoke_raw(
            &uri,
            &method,
            Some(&args_buffer),
            Some(&state.env),
            None,
        );

        match result {
            Ok(r) => {
                let subinvoke_state = SubinvokeImplementationState {
                    result: Some(r),
                    args: args_buffer,
                    error: None
                };
                state.subinvoke_implementation = Some(subinvoke_state);
                Ok(vec![Value::I32(1)])
            }
            Err(e) => {
                let error = format!("interface implementation subinvoke failed for uri: {interface} with error: {e}");
                let subinvoke_state = SubinvokeImplementationState {
                    result: None,
                    args: args_buffer,
                    error: Some(error)
                };
                state.subinvoke_implementation = Some(subinvoke_state);
                Ok(vec![Value::I32(0)])
            }
        }
    };

    let wrap_subinvoke_implementation = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_signature,
        subinvoke_implementation,
    );

    let subinvoke_implementation_result_len_signature = FunctionType::new(vec![], vec![Type::I32]);

    let subinvoke_implementation_result_len =
        move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
            let mutable_context = context.as_mut();
            let mutable_state = mutable_context.data().lock().unwrap();

            let implementation = mutable_state.subinvoke_implementation.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_result_len: subinvoke_implementation is not set"
                ))?;

            let implementation_result = implementation.result.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_result_len: subinvoke_implementation.result is not set"
                ))?;

            Ok(vec![Value::I32(implementation_result.len() as i32)])
        };

    let wrap_subinvoke_implementation_result_len = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_result_len_signature,
        subinvoke_implementation_result_len,
    );

    let subinvoke_implementation_result_signature = FunctionType::new(vec![Type::I32], vec![]);

    let subinvoke_implementation_result =
        move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
            let mutable_context = context.as_mut();
            let mutable_state = mutable_context.data().lock().unwrap();
            let memory = mutable_state.memory.as_ref().unwrap();
            let pointer = values[0].unwrap_i32() as u64;

            let implementation = mutable_state.subinvoke_implementation.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_result: subinvoke_implementation is not set"
                ))?;
            
            let implementation_result = implementation.result.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_result: subinvoke_implementation.result is not set"
                ))?;
            
            memory
                .view(&mutable_context)
                .write(pointer, implementation_result)
                .map(|_| vec![])
                .map_err(|e| RuntimeError::new(e.to_string()))
        };

    let wrap_subinvoke_implementation_result = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_result_signature,
        subinvoke_implementation_result,
    );

    let subinvoke_implementation_error_len_signature = FunctionType::new(vec![], vec![Type::I32]);

    let subinvoke_implementation_error_len =
        move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
            let mutable_context = context.as_mut();
            let mutable_state = mutable_context.data().lock().unwrap();

            let implementation = mutable_state.subinvoke_implementation.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_error_len: subinvoke_implementation is not set"
                ))?;

            let implementation_error = implementation.error.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_error_len: subinvoke_implementation.error is not set"
                ))?;

            Ok(vec![Value::I32(implementation_error.as_bytes().len() as i32)])
        };

    let wrap_subinvoke_implementation_error_len = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_error_len_signature,
        subinvoke_implementation_error_len,
    );

    let subinvoke_implementation_error_signature = FunctionType::new(vec![Type::I32], vec![]);

    let subinvoke_implementation_error =
        move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
            let mutable_context = context.as_mut();
            let mutable_state = mutable_context.data().lock().unwrap();
            let memory = mutable_state.memory.as_ref().unwrap();
            let pointer = values[0].unwrap_i32() as u64;

            let implementation = mutable_state.subinvoke_implementation.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_error: subinvoke_implementation is not set"
                ))?;

            let implementation_error = implementation.error.as_ref()
                .ok_or(RuntimeError::new(
                    "__wrap_subinvoke_implementation_error: subinvoke_implementation.error is not set"
                ))?;
            
            memory
                .view(&mutable_context)
                .write(pointer, implementation_error.as_bytes())
                .map(|_| vec![])
                .map_err(|e| RuntimeError::new(e.to_string()))
        };

    let wrap_subinvoke_implementation_error = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_error_signature,
        subinvoke_implementation_error,
    );

    let get_implementations_signature =
        FunctionType::new(vec![Type::I32, Type::I32], vec![Type::I32]);

    let get_implementations = move |context: FunctionEnvMut<Arc<Mutex<State>>>,
                                    values: &[Value]| {
        let pointer = values[0].unwrap_i32() as u64;
        let length = values[1].unwrap_i32() as usize;

        let async_context = Arc::new(Mutex::new(context));

        let mut context = async_context.lock().unwrap();
        let mutable_context = context.as_mut();
        let mut state = mutable_context.data().lock().unwrap();

        let memory = state.memory.as_ref().unwrap();
        let mut uri_bytes = Vec::with_capacity(length);
        memory
            .view(&mutable_context)
            .read(pointer, &mut uri_bytes)
            .unwrap();
        let uri = String::from_utf8(uri_bytes).unwrap();
        let result = state.invoker.get_implementations(&uri.try_into().unwrap());

        let result = result.as_ref().map_err(|e| RuntimeError::new(e.to_string()))?;

        let implementations = result
            .into_iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>();
        let encoded_implementations = to_vec(&implementations);
        state.get_implementations_result = Some(encoded_implementations.unwrap());

        if !state
            .get_implementations_result
            .as_ref()
            .unwrap()
            .is_empty()
        {
            return Ok(vec![Value::I32(1)]);
        }
        Ok(vec![Value::I32(0)])
    };

    let wrap_get_implementation = Function::new_with_env(
        store,
        &context,
        get_implementations_signature,
        get_implementations,
    );

    let get_implementation_result_len_signature = FunctionType::new(vec![], vec![Type::I32]);

    let get_implementation_result_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>,
                                              _: &[Value]| {
        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();

        let result = state.get_implementations_result.as_ref()
            .ok_or(RuntimeError::new(
                "__wrap_get_implementation_result_len: get_implementation_result is not set"
            ))?;

        Ok(vec![Value::I32(result.len() as i32)])
    };

    let wrap_get_implementation_result_len = Function::new_with_env(
        store,
        &context,
        get_implementation_result_len_signature,
        get_implementation_result_len,
    );

    let get_implementation_result_signature = FunctionType::new(vec![Type::I32], vec![]);

    let get_implementation_result = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>,
                                          values: &[Value]| {
        let pointer = values[0].unwrap_i32() as u64;

        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();
        let memory = state.memory.as_ref().unwrap();

        let result = state.get_implementations_result.as_ref()
            .ok_or(RuntimeError::new(
                "__wrap_get_implementation_result: get_implementation_result is not set"
            ))?;

        memory
            .view(&mutable_context)
            .write(pointer, result)
            .map(|_| vec![])
            .map_err(|e| {
                RuntimeError::new(format!(
                    "__wrap_get_implementation_result: failed to write to memory: {}",
                    e
                ))
            })
    };

    let wrap_get_implementation_result = Function::new_with_env(
        store,
        &context,
        get_implementation_result_signature,
        get_implementation_result,
    );

    let load_env_signature = FunctionType::new(vec![Type::I32], vec![]);

    let load_env = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let pointer = values[0].unwrap_i32() as u64;

        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();
        let memory = state.memory.as_ref().unwrap();

        memory
            .view(&mutable_context)
            .write(pointer, &state.env.to_vec())
            .map_err(|e| RuntimeError::new(format!("__wrap_load_env: failed to write to memory: {}", e)))?;

        Ok(vec![])
    };

    let wrap_load_env = Function::new_with_env(store, &context, load_env_signature, load_env);

    let debug_log_signature = FunctionType::new(vec![Type::I32, Type::I32], vec![]);
    let debug_log = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let msg_offset = values[0].unwrap_i32() as u64;
        let msg_length = values[1].unwrap_i32() as u32;
        let mut msg_buffer: Vec<u8> = Vec::with_capacity(msg_length as usize);

        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();
        let memory = state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);

        memory_view
            .read(msg_offset, &mut msg_buffer)
            .unwrap();
        let msg = String::from_utf8(msg_buffer).unwrap();
        println!("{}", format!("__wrap_debug_log: {msg}"));
        Ok(vec![])
    };

    let wrap_debug_log = Function::new_with_env(store, &context, debug_log_signature, debug_log);

    imports! {
        "wrap" => {
            "__wrap_invoke_args" => wrap_invoke_args,
            "__wrap_invoke_result" => wrap_invoke_result,
            "__wrap_invoke_error" => wrap_invoke_error,
            "__wrap_abort" => wrap_abort,
            "__wrap_subinvoke" => wrap_subinvoke,
            "__wrap_subinvoke_result_len" => wrap_subinvoke_result_len,
            "__wrap_subinvoke_result" => wrap_subinvoke_result,
            "__wrap_subinvoke_error_len" => wrap_subinvoke_error_len,
            "__wrap_subinvoke_error" => wrap_subinvoke_error,
            "__wrap_subinvokeImplementation" => wrap_subinvoke_implementation,
            "__wrap_subinvokeImplementation_result_len" => wrap_subinvoke_implementation_result_len,
            "__wrap_subinvokeImplementation_result" => wrap_subinvoke_implementation_result,
            "__wrap_subinvokeImplementation_error_len" => wrap_subinvoke_implementation_error_len,
            "__wrap_subinvokeImplementation_error" => wrap_subinvoke_implementation_error,
            "__wrap_getImplementations" => wrap_get_implementation,
            "__wrap_getImplementations_result" => wrap_get_implementation_result,
            "__wrap_getImplementations_result_len" => wrap_get_implementation_result_len,
            "__wrap_load_env" => wrap_load_env,
            "__wrap_debug_log" => wrap_debug_log
        },
        "env" => {
            "memory" => memory,
        }
    }
}
