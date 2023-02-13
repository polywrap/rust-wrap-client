use std::sync::{Mutex, Arc};

use wasmer::{Imports, imports, Memory, FunctionEnvMut, Function, AsStoreMut, FunctionType, Value, Type, FunctionEnv, Store};
use crate::{error::WrapperError, wasm_runtime::instance::State};

// fn read_from_memory() -> {

// }

pub fn create_imports(
    memory: Arc<Mutex<Memory>>,
    store: Store,
    state: State
) -> (Imports, Store) {
    let memory = Arc::clone(&memory);

    let invoke_args = move |mut state: FunctionEnvMut<State>, values: &[Value]| {
        let [method_ptr, args_ptr] = values;        
        let method_ptr = match method_ptr {
            Value::I64(p) => *p,
            _ => panic!("method is None")
        };
        
        let args_ptr = match args_ptr {
            Value::I64(p) => *p,
            _ => panic!("args is None")
        };
        
        let memory = memory.lock().unwrap();
        let memory = memory.view(&state.as_store_mut());
        let state = state.data_mut();
        memory.write(method_ptr.try_into().unwrap(), state.method.as_slice());
        memory.write(args_ptr.try_into().unwrap(), state.args.as_slice());

        Ok(vec![Value::I32(1)])
    };

    let invoke_args_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![Type::I32]
    );

    let state = FunctionEnv::new(&mut store, state);
    let invoke_args_function = Function::new_with_env(
        &mut store,
        &state,
        invoke_args_signature,
        invoke_args
    );

    let imports = imports! {
        "wrap" => {
            "_wrap_invoke_args" => invoke_args_function
        },
        "env" => {
            "memory" => *memory.lock().unwrap(),
        }
    };

    (imports, store)
}