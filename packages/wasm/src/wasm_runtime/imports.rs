use std::sync::{Mutex, Arc};

use wasmer::{Imports, imports, Memory, FunctionEnvMut, Function, FunctionType, Value, Type, FunctionEnv, Store};
use crate::{wasm_runtime::instance::State};

// fn read_from_memory() -> {

// }

pub fn create_imports(
    memory: Arc<Mutex<Memory>>,
    store: &mut Store,
    state: Arc<Mutex<State>>
) -> Imports {
    let memory_cloned = Arc::clone(&memory);
    let invoke_args = move |mut state: FunctionEnvMut<Arc<Mutex<State>>>, values: &[Value]| {

        let method_ptr = &values[0];        
        let args_ptr = &values[1];        
        let method_ptr = match method_ptr {
            Value::I64(p) => *p,
            _ => panic!("method is None")
        };
        
        let args_ptr = match args_ptr {
            Value::I64(p) => *p,
            _ => panic!("args is None")
        };

        // let mut state = state.data_mut().lock().unwrap();
        // state.method = vec![];
        // let method_pr

        let invocation_memory = memory_cloned.lock().unwrap();
        // let t = Box::new(store);
    
        // invocation_memory.view(store.as_mut());
        // let memory = memory.view(&state.as_store_mut());
        // let state = state.data_mut();
        // memory.write(method_ptr.try_into().unwrap(), state.method.as_slice());
        // memory.write(args_ptr.try_into().unwrap(), state.args.as_slice());

        Ok(vec![Value::I32(1)])
    };

    let invoke_args_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![Type::I32]
    );

    let state = FunctionEnv::new(store, state);
    let invoke_args_function = Function::new_with_env(
        store,
        &state,
        invoke_args_signature,
        invoke_args
    );

    imports! {
        "wrap" => {
            "__wrap_invoke_args" => invoke_args_function
        },
        "env" => {
            // "memory" => *memory.lock().unwrap(),
        }
    }
}