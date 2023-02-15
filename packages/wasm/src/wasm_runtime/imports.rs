use std::sync::{Mutex, Arc};

use wasmer::{Imports, imports, Memory, FunctionEnvMut, Function, FunctionType, Value, Type, FunctionEnv, Store, StoreRef};

use super::instance::State;

pub fn create_imports(
    memory: Memory,
    store: &mut Store,
    state: Arc<Mutex<State>>
) -> Imports {
    // let memory_view = memory.view(store);
    // let memory_arc  = Arc::new(Mutex::new(memory_view));

    let invoke_args = move |mut state: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {
        let method_ptr = match &values[0] {
            Value::I64(p) => *p,
            _ => panic!("method is None")
        };

        let args_ptr = match &values[1] {
            Value::I64(p) => *p,
            _ => panic!("args is None")
        };

        let z = state.as_mut();
        let mutable_state = z.data().lock().unwrap();
        // mutable_state.
        // let mut memory = memory_view.lock().unwrap();
        if mutable_state.method.is_empty() {
            (mutable_state.abort)("__wrap_invoke_args: method is not set".to_string());
        }

        if mutable_state.args.is_empty() {
            (mutable_state.abort)("__wrap_invoke_args: args is not set".to_string());
        }
        let memory = mutable_state.memory.as_ref().unwrap();
        let memory_view = memory.view(&z);
        memory_view.write(method_ptr.try_into().unwrap(), &mutable_state.method).unwrap();
        // let memory = state.
        // memory_arc.lock().unwrap().write(method_ptr as usize, &state.method);
        Ok(vec![])
    };

    let invoke_args_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![Type::I32]
    );

    let shared_state = FunctionEnv::new(store, state);
    let invoke_args_function = Function::new_with_env(
        store,
        &shared_state,
        invoke_args_signature,
        invoke_args
    );

    imports! {
        "wrap" => {
            "__wrap_invoke_args" => invoke_args_function
        },
        "env" => {
            "memory" => memory,
        }
    }
}