use std::sync::{Mutex, Arc};

use wasmer::{Imports, imports, Memory, FunctionEnvMut, Function, AsStoreMut, FunctionType, Value, Type, FunctionEnv};
use crate::{error::WrapperError, wasm_runtime::instance::State};

// fn read_from_memory() -> {

// }

pub fn create_imports(memory: Arc<Mutex<Memory>>, state: State) -> Imports {
    let memory = Arc::clone(&memory);

    let invoke_args = move |mut state: State, method_ptr: u32, args_ptr: u32| {
        let memory = memory.lock().unwrap();
        // let t = memory.view();
        0
    };

    let invoke_args_signature = FunctionType::new(
        vec![Type::I32, Type::I32],
        vec![Type::I32]
    );

    // let s = FunctionEnv::
    let invoke_args_function = Function::new_with_env(
        &mut store,
        &state,
        invoke_args_signature,
        invoke_args
    );

    imports! {
        "wrap" => {
            "_wrap_invoke_args" => invoke_args_function
        },
        "env" => {
            "memory" => *memory.lock().unwrap(),
        }
    }
}