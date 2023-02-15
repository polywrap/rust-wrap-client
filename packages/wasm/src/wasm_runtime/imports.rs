use std::sync::{Mutex, Arc};

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
        let mutable_context = context.as_mut();
        let mut mutable_state = mutable_context.data().lock().unwrap();
        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&mutable_context);

        Ok(vec![])

    };


    let wrap_abort = Function::new_with_env(
        store,
        &context,
        invoke_abort_signature,
        abort
    );

    imports! {
        "wrap" => {
            "__wrap_invoke_args" => wrap_invoke_args,
            "__wrap_invoke_result" => wrap_invoke_result,
            "__wrap_invoke_error" => wrap_invoke_error,
            "__wrap_abort" => wrap_abort,
        },
        "env" => {
            "memory" => memory,
        }
    }
}