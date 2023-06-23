use std::sync::{Arc, Mutex};

use polywrap_core::invoker::Invoker;
use wasmer::{Instance, Memory, MemoryType, Module, Store, Value};

use crate::error::WrapperError;

use super::imports::create_imports;

#[derive(Default)]
pub struct InvokeState {
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
}

pub struct SubinvokeImplementationState {
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
    pub args: Vec<u8>,
}

pub struct State {
    pub method: Vec<u8>,
    pub args: Vec<u8>,
    pub env: Vec<u8>,
    pub invoke: InvokeState,
    pub subinvoke: InvokeState,
    pub invoker: Arc<dyn Invoker>,
    pub get_implementations_result: Option<Vec<u8>>,
    pub subinvoke_implementation: Option<SubinvokeImplementationState>,
    pub memory: Option<Memory>,
}

impl State {
    pub fn new(
        invoker: Arc<dyn Invoker>,
        method: &str,
        args: Vec<u8>,
        env: Vec<u8>,
    ) -> Self {
        Self {
            method: method.as_bytes().to_vec(),
            args,
            env,
            invoke: InvokeState::default(),
            subinvoke: InvokeState::default(),
            invoker,
            get_implementations_result: None,
            subinvoke_implementation: None,
            memory: None,
        }
    }
}

pub struct WasmInstance {
    instance: Instance,
    pub store: Store,
}

impl WasmInstance {
    pub fn new(
        module: &Module,
        memory_initial_limits: u8,
        state: Arc<Mutex<State>>,
    ) -> Result<Self, WrapperError> {
        let mut store = Store::default();
        let memory = WasmInstance::create_memory(&mut store, memory_initial_limits)?;

        state.lock().unwrap().memory = Some(memory.clone());

        let imports = create_imports(memory, &mut store, state);

        let instance = Instance::new(&mut store, &module, &imports)
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        Ok(Self { instance, store })
    }

    pub fn get_memory_initial_limits(module: &[u8]) -> Result<u8, WrapperError> {
        const ENV_MEMORY_IMPORTS_SIGNATURE: [u8; 11] = [
            0x65, 0x6e, 0x76, 0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x02,
        ];

        let idx = module
            .windows(ENV_MEMORY_IMPORTS_SIGNATURE.len())
            .position(|window| window == ENV_MEMORY_IMPORTS_SIGNATURE);

        match idx {
            Some(idx) => {
                let memory_initial_limits = module[idx + ENV_MEMORY_IMPORTS_SIGNATURE.len() + 1];

                Ok(memory_initial_limits)
            }
            None => Err(WrapperError::ModuleReadError(
                r#"Unable to find Wasm memory import section.
                Modules must import memory from the "env" module's
                "memory" field like so:
                (import "env" "memory" (memory (;0;) #))"#
                    .to_string(),
            )),
        }
    }

    pub fn create_memory(
        store: &mut Store,
        memory_initial_limits: u8,
    ) -> Result<Memory, WrapperError> {
        let memory = Memory::new(
            store,
            MemoryType::new(memory_initial_limits as u32, None, false),
        )?;

        Ok(memory)
    }

    pub fn call_export(&mut self, name: &str, params: &[Value]) -> Result<bool, WrapperError> {
        let export = self.instance.exports.get_function(name)
            .map_err(|_| WrapperError::WasmRuntimeError(format!("Export {name} not found")))?;

        let result = export.call(&mut self.store, params)
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let result = result.to_vec();
        
        // If the result is true (1), then the call was successful
        if let Some(result) = result.get(0).and_then(|x| x.i32()) {
            if result == 1 {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
