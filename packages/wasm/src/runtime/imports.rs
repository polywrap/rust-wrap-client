use std::sync::{Mutex, Arc};


use polywrap_core::uri::Uri;
use wasmer::{Imports, imports, Memory, FunctionEnvMut, Function, FunctionType, Value, Type, FunctionEnv, Store};

use super::instance::State;

pub fn create_imports(
    memory: Memory,
    store: &mut Store,
    state: Arc<Mutex<State>>
) -> Imports {
    let context = FunctionEnv::new(store, state);

    let invoke_args = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let method_ptr = values[0].unwrap_i32() as u32;
        let args_ptr = values[1].unwrap_i32() as u32;

        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        if mutable_state.method.is_empty() {
            (mutable_state.abort)("__wrap_invoke_args: method is not set".to_string());
        }

        if mutable_state.args.is_empty() {
            (mutable_state.abort)("__wrap_invoke_args: args is not set".to_string());
        }

        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);

        memory_view.write(method_ptr.try_into().unwrap(), &mutable_state.method).unwrap();
        memory_view.write(args_ptr.try_into().unwrap(), &mutable_state.args).unwrap();
        Ok(vec![])
    };

    let invoke_args_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![]
    );

    let wrap_invoke_args = Function::new_with_env(
        store,
        &context,
        invoke_args_signature,
        invoke_args
    );

    let invoke_result = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mut mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);
        let offset = values[0].unwrap_i32() as u32;
        let length = values[1].unwrap_i32() as u32;

        let mut buffer: Vec<u8> = vec![0; length as usize];
        memory_view.read(offset.try_into().unwrap(), &mut buffer).unwrap();
        mutable_state.invoke.result = Some(buffer);
        Ok(vec![])
    };

    let invoke_result_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![]
    );

    let wrap_invoke_result = Function::new_with_env(
        store,
        &context,
        invoke_result_signature,
        invoke_result
    );

    let invoke_result_error_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![]
    );

    let invoke_error = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mut mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);
        let offset = values[0].unwrap_i32() as u32;
        let length = values[1].unwrap_i32() as u32;

        let mut buffer: Vec<u8> = vec![0; length as usize];
        memory_view.read(offset.try_into().unwrap(), &mut buffer).unwrap();
        mutable_state.invoke.result = Some(buffer);
        Ok(vec![])
    };

    let wrap_invoke_error = Function::new_with_env(
        store,
        &context,
        invoke_result_error_signature,
        invoke_error
    );

    let invoke_abort_signature = FunctionType::new(
        vec![Type::I32, Type::I32, Type::I32, Type::I32, Type::I32, Type::I32],
        vec![]
    );

    let abort = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let msg_offset = values[0].unwrap_i32() as u32;
        let msg_length = values[1].unwrap_i32() as u32;
        let file_offset = values[2].unwrap_i32() as u32;
        let file_length = values[3].unwrap_i32() as u32;
        let line = values[4].unwrap_i32() as u32;
        let column = values[5].unwrap_i32() as u32;

        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();
        let memory = state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);

        let mut msg_buffer: Vec<u8> = vec![0; msg_length as usize];
        let mut file_buffer: Vec<u8> = vec![0; file_length as usize];

        memory_view.read(msg_offset.try_into().unwrap(), &mut msg_buffer).unwrap();
        memory_view.read(file_offset.try_into().unwrap(), &mut file_buffer).unwrap();

        let msg = String::from_utf8(msg_buffer).unwrap();
        let file = String::from_utf8(file_buffer).unwrap();
        (state.abort)(format!(
            "__wrap_abort: {msg}\nFile: {file}\nLocation: [{line},{column}]",
            msg = msg,
            file = file,
            line = line,
            column = column
        ));

        Ok(vec![])
    };

    let wrap_abort = Function::new_with_env(
        store,
        &context,
        invoke_abort_signature,
        abort
    );

    let subinvoke_signature = FunctionType::new(
        vec![Type::I32, Type::I32, Type::I32, Type::I32, Type::I32, Type::I32],
        vec![Type::I32]
    );

    let subinvoke = move |context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let uri_ptr = values[0].unwrap_i32() as u32;
        let uri_len = values[1].unwrap_i32() as u32;
        let method_ptr = values[2].unwrap_i32() as u32;
        let method_len = values[3].unwrap_i32() as u32;
        let args_ptr = values[4].unwrap_i32() as u32;
        let args_len = values[5].unwrap_i32() as u32;

        let async_context = Arc::new(Mutex::new(context));
        let mut context = async_context.lock().unwrap();
        let mutable_context = context.as_mut();
        let mut state = mutable_context.data().lock().unwrap();

        let memory = state.memory.as_ref().unwrap();
        let mut uri_buffer: Vec<u8> = vec![0; uri_len as usize];
        let mut method_buffer: Vec<u8> = vec![0; method_len as usize];
        let mut args_buffer: Vec<u8> = vec![0; args_len as usize];

        memory.view(&mutable_context).read(uri_ptr.try_into().unwrap(), &mut uri_buffer).unwrap();
        memory.view(&mutable_context).read(method_ptr.try_into().unwrap(), &mut method_buffer).unwrap();
        memory.view(&mutable_context).read(args_ptr.try_into().unwrap(), &mut args_buffer).unwrap();

        let uri: Uri = String::from_utf8(uri_buffer).unwrap().try_into().unwrap();
        let method = String::from_utf8(method_buffer).unwrap();

        let env = if !state.env.is_empty() {
          Some(polywrap_msgpack::decode::<serde_json::Value>(&state.env).unwrap())
        } else {
          None
        };

        let result = state.invoker.invoke_raw(
            &uri,
            &method,
            Some(&args_buffer),
            env,
            None
        );

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

    let wrap_subinvoke = Function::new_with_env(
        store,
        &context,
        subinvoke_signature,
        subinvoke
    );

    let subinvoke_result_len_signature = FunctionType::new(
        vec![],
        vec![Type::I32]
    );

    let subinvoke_result_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        if mutable_state.subinvoke.result.is_none() {
            (mutable_state.abort)(
                "__wrap_subinvoke_result_len: subinvoke.result is not set".to_string(),
            );
        }
        let length = mutable_state.subinvoke.result.as_deref().unwrap().len();
        Ok(vec![Value::I32(length as i32)])
    };

    let wrap_subinvoke_result_len = Function::new_with_env(
        store,
        &context,
        subinvoke_result_len_signature,
        subinvoke_result_len
    );

    let subinvoke_result_signature = FunctionType::new(
        vec![Type::I32],
        vec![]
    );

    let subinvoke_result = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();

        let pointer = values[0].unwrap_i32() as u32;
        if let Some(result) = &mutable_state.subinvoke.result {
            memory.view(&mutable_context).write(pointer as u64, result).unwrap();
        } else {
            (mutable_state.abort)(
                "__wrap_subinvoke_result: subinvoke.result is not set".to_string(),
            );
        }
        Ok(vec![])
    };

    let wrap_subinvoke_result = Function::new_with_env(
        store,
        &context,
        subinvoke_result_signature,
        subinvoke_result
    );

    let subinvoke_error_len_signature = FunctionType::new(
        vec![],
        vec![Type::I32]
    );

    let subinvoke_error_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        if mutable_state.subinvoke.error.is_none() {
            (mutable_state.abort)(
                "__wrap_subinvoke_error_len: subinvoke.error is not set".to_string(),
            );
        }
        let length = mutable_state.subinvoke.error.as_deref().unwrap().len();
        Ok(vec![Value::I32(length as i32)])
    };

    let wrap_subinvoke_error_len = Function::new_with_env(
        store,
        &context,
        subinvoke_error_len_signature,
        subinvoke_error_len
    );

    let subinvoke_error_signature = FunctionType::new(
        vec![Type::I32],
        vec![]
    );

    let subinvoke_error = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();

        let pointer = values[0].unwrap_i32() as u32;
        if let Some(error) = &mutable_state.subinvoke.error {
            memory.view(&mutable_context).write(pointer as u64, error.as_bytes()).unwrap();
        } else {
            (mutable_state.abort)(
                "__wrap_subinvoke_error: subinvoke.error is not set".to_string(),
            );
        }
        Ok(vec![])
    };

    let wrap_subinvoke_error = Function::new_with_env(
        store,
        &context,
        subinvoke_error_signature,
        subinvoke_error
    );

    let subinvoke_implementation_signature = FunctionType::new(
        vec![Type::I32, Type::I32, Type::I32, Type::I32, Type::I32, Type::I32, Type::I32, Type::I32],
        vec![Type::I32]
    );

    let subinvoke_implementation = move |context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let interface_ptr = values[0].unwrap_i32() as u32;
        let interface_len = values[1].unwrap_i32() as u32;
        let impl_uri_ptr = values[2].unwrap_i32() as u32;
        let impl_uri_len = values[3].unwrap_i32() as u32;
        let method_ptr = values[4].unwrap_i32() as u32;
        let method_len = values[5].unwrap_i32() as u32;
        let args_ptr = values[6].unwrap_i32() as u32;
        let args_len = values[7].unwrap_i32() as u32;

        let async_context = Arc::new(Mutex::new(context));

        
            let mut context = async_context.lock().unwrap();
            let mutable_context = context.as_mut();
            let mut state = mutable_context.data().lock().unwrap();

            let mut interface_buffer = vec![0; interface_len.try_into().unwrap()];
            let mut impl_uri_buffer = vec![0; impl_uri_len.try_into().unwrap()];
            let mut method_buffer = vec![0; method_len.try_into().unwrap()];
            let mut args_buffer = vec![0; args_len.try_into().unwrap()];
            
            let memory = state.memory.as_ref().unwrap();
            memory.view(&mutable_context).read(interface_ptr.try_into().unwrap(), &mut interface_buffer).unwrap();
            memory.view(&mutable_context).read(impl_uri_ptr.try_into().unwrap(), &mut impl_uri_buffer).unwrap();
            memory.view(&mutable_context).read(method_ptr.try_into().unwrap(), &mut method_buffer).unwrap();
            memory.view(&mutable_context).read(args_ptr.try_into().unwrap(), &mut args_buffer).unwrap();

            let interface = String::from_utf8(interface_buffer).unwrap();
            let uri = String::from_utf8(impl_uri_buffer).unwrap();
            let method = String::from_utf8(method_buffer).unwrap();
            let env = if !state.env.is_empty() {
              Some(polywrap_msgpack::decode::<serde_json::Value>(&state.env).unwrap())
            } else {
              None
            };

            let result = state.invoker.invoke_raw(
                &uri.try_into().unwrap(),
                &method,
                Some(&args_buffer),
                env,
                None
            );
    
            match result {
                Ok(r) => {
                    state.subinvoke.result = Some(r);
                    Ok(vec![Value::I32(1)])
                }
                Err(e) => {
                    let error = format!("interface implementation subinvoke failed for uri: {} with error: {}", interface, e);
                    state.subinvoke.error = Some(error);
                    Ok(vec![Value::I32(0)])
                }
            }
    };

    let wrap_subinvoke_implementation = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_signature,
        subinvoke_implementation
    );

    let subinvoke_implementation_result_len_signature = FunctionType::new(
        vec![],
        vec![Type::I32]
    );

    let subinvoke_implementation_result_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        if let Some (implementation) = &mutable_state.subinvoke_implementation {
            if let Some(r) = &implementation.result {
                let length = r.len();
                Ok(vec![Value::I32(length as i32)])
            } else {
                (mutable_state.abort)(
                    "__wrap_subinvoke_implementation_result_len: subinvoke_implementation.result is not set".to_string(),
                );
                Ok(vec![Value::I32(0)])

            }
        } else {
            (mutable_state.abort)(
                "__wrap_subinvoke_implementation_result_len: subinvoke_implementation is not set".to_string(),
            );
            Ok(vec![Value::I32(0)])
        }
    };

    let wrap_subinvoke_implementation_result_len = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_result_len_signature,
        subinvoke_implementation_result_len
    );

    let subinvoke_implementation_result_signature = FunctionType::new(
        vec![Type::I32],
        vec![],
    );

    let subinvoke_implementation_result = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();
        let pointer = values[0].unwrap_i32() as u32;

        if let Some (implementation) = &mutable_state.subinvoke_implementation {
            if let Some(r) = &implementation.result {
                memory.view(&mutable_context).write(pointer.try_into().unwrap(), r).unwrap();
            } else {
                (mutable_state.abort)(
                    "__wrap_subinvoke_implementation_result: subinvoke_implementation.result is not set".to_string(),
                );
            };
        } else {
            (mutable_state.abort)(
                "__wrap_subinvoke_implementation_result: subinvoke_implementation is not set".to_string(),
            );
        };
        Ok(vec![])
    };

    let wrap_subinvoke_implementation_result = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_result_signature,
        subinvoke_implementation_result
    );

    let subinvoke_implementation_error_len_signature = FunctionType::new(
        vec![],
        vec![Type::I32]
    );

    let subinvoke_implementation_error_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();

        if let Some (implementation) = &mutable_state.subinvoke_implementation {
            if let Some(r) = &implementation.error {
                let length = r.as_bytes().len();
                Ok(vec![Value::I32(length as i32)])
            } else {
                (mutable_state.abort)(
                    "__wrap_subinvoke_implementation_error_len: subinvoke_implementation.error is not set".to_string(),
                );
                Ok(vec![Value::I32(0)])

            }
        } else {
            (mutable_state.abort)(
                "__wrap_subinvoke_implementation_error_len: subinvoke_implementation is not set".to_string(),
            );
            Ok(vec![Value::I32(0)])
        }
    };

    let wrap_subinvoke_implementation_error_len = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_error_len_signature,
        subinvoke_implementation_error_len
    );

    let subinvoke_implementation_error_signature = FunctionType::new(
        vec![Type::I32],
        vec![],
    );

    let subinvoke_implementation_error = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let mutable_context = context.as_mut();
        let mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();
        let pointer = values[0].unwrap_i32() as u32;

        if let Some (implementation) = &mutable_state.subinvoke_implementation {
            if let Some(r) = &implementation.error {
                memory.view(&mutable_context).write(pointer.try_into().unwrap(), r.as_bytes()).unwrap();
            } else {
                (mutable_state.abort)(
                    "__wrap_subinvoke_implementation_error: subinvoke_implementation.error is not set".to_string(),
                );
            };
        } else {
            (mutable_state.abort)(
                "__wrap_subinvoke_implementation_error: subinvoke_implementation is not set".to_string(),
            );
        };
        Ok(vec![])
    };

    let wrap_subinvoke_implementation_error = Function::new_with_env(
        store,
        &context,
        subinvoke_implementation_error_signature,
        subinvoke_implementation_error
    );

    let get_implementations_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![Type::I32],
    );

    let get_implementations = move |context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let pointer = values[0].unwrap_i32() as u32;
        let length = values[1].unwrap_i32() as u32;

        let async_context = Arc::new(Mutex::new(context));

        let mut context = async_context.lock().unwrap();
        let mutable_context = context.as_mut();
        let mut state = mutable_context.data().lock().unwrap();

        let memory = state.memory.as_ref().unwrap();
        let mut uri_bytes = vec![0; length as usize];
        memory.view(&mutable_context).read(pointer.try_into().unwrap(), &mut uri_bytes).unwrap();
        let uri = String::from_utf8(uri_bytes).unwrap();
        println!("URI: {}", length);
        let result = state.invoker.get_implementations(uri.try_into().unwrap());

        if result.is_err() {
            let result = result.as_ref().err().unwrap();
            (state.abort)(result.to_string());
            return Ok(vec![Value::I32(0)]);
        }

        let implementations = &result.unwrap().into_iter().map(|u| u.to_string()).collect::<Vec<String>>();
        let encoded_implementations = polywrap_msgpack::rmp_serde::encode::to_vec_named(implementations);      
        state.get_implementations_result = Some(encoded_implementations.unwrap());

        if !state.get_implementations_result.as_ref().unwrap().is_empty() {
            return Ok(vec![Value::I32(1)]);
        }
        Ok(vec![Value::I32(0)])
    };

    let wrap_get_implementation = Function::new_with_env(
        store,
        &context,
        get_implementations_signature,
        get_implementations
    );

    let get_implementation_result_len_signature = FunctionType::new(
        vec![],
        vec![Type::I32],
    );

    let get_implementation_result_len = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, _: &[Value]| {
        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();

        if let Some(r) = &state.get_implementations_result {
            let length = r.len();
            println!("POINTER LEN: {}", length);
            Ok(vec![Value::I32(length as i32)])
        } else {
            (state.abort)(
                "__wrap_get_implementation_result_len: get_implementation_result is not set".to_string(),
            );
            Ok(vec![Value::I32(0)])
        }
    };

    let wrap_get_implementation_result_len = Function::new_with_env(
        store,
        &context,
        get_implementation_result_len_signature,
        get_implementation_result_len
    );


    let get_implementation_result_signature = FunctionType::new(
        vec![Type::I32],
        vec![],
    );

    let get_implementation_result = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let pointer = values[0].unwrap_i32() as u32;

        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();
        let memory = state.memory.as_ref().unwrap();

        println!("POINTER: {}", pointer);

        if let Some(r) = &state.get_implementations_result {
            memory.view(&mutable_context).write(pointer.try_into().unwrap(), r).unwrap();
        } else {
            (state.abort)(
                "__wrap_get_implementation_result: get_implementation_result is not set".to_string(),
            );
        }
        Ok(vec![])
    };

    let wrap_get_implementation_result = Function::new_with_env(
        store,
        &context,
        get_implementation_result_signature,
        get_implementation_result
    );

    let load_env_signature = FunctionType::new(
        vec![Type::I32],
        vec![],
    );

    let load_env = move |mut context: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let pointer = values[0].unwrap_i32() as u32;

        let mutable_context = context.as_mut();
        let state = mutable_context.data().lock().unwrap();
        let memory = state.memory.as_ref().unwrap();

        memory.view(&mutable_context).write(pointer.try_into().unwrap(), &state.env.to_vec()).unwrap();

        Ok(vec![])
    };

    let wrap_load_env = Function::new_with_env(
        store,
        &context,
        load_env_signature,
        load_env
    );

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
        },
        "env" => {
            "memory" => memory,
        }
    }
}