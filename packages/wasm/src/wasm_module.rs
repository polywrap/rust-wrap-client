use std::sync::{Mutex, Arc};

use bytes::Bytes;
use wasmer::{Module, Store};

use crate::{runtime::{instance::{WasmInstance, State}}, error::WrapperError};

pub enum WasmModule {
    WasmByteCode(Vec<u8>),
    Serialized { compiled_bytes: Bytes, memory_initial_limits: u8 },
    Compiled(CompiledWasmModule)
}

impl WasmModule {
    pub fn compile(self) -> Result<CompiledWasmModule, WrapperError> {
        Ok(match self {
            WasmModule::WasmByteCode(bytes) => CompiledWasmModule::from_byte_code(&bytes)?,
            WasmModule::Serialized { compiled_bytes, memory_initial_limits } => {
                let store = Store::default();
                let wasmer_module = Module::deserialize_checked(&store, compiled_bytes).map_err(
                    |e| WrapperError::WasmRuntimeError(e.to_string())
                )?;
                
                CompiledWasmModule {
                    module: wasmer_module,
                    memory_initial_limits,
                    store,
                }
            },
            WasmModule::Compiled(compiled_module) => compiled_module,
        })
    }
}

pub struct CompiledWasmModule {
    pub module: Module,
    pub memory_initial_limits: u8,
    pub store: Store,
}

impl CompiledWasmModule {
    pub fn create_instance(&self, state: Arc<Mutex<State>>) -> Result<WasmInstance, WrapperError> {
        let instance = WasmInstance::new(&self.module, self.memory_initial_limits, state)?;
        Ok(instance)
    }

    pub fn from_byte_code(bytes: &[u8]) -> Result<Self, WrapperError> {
        let store = Store::default();
        let wasmer_module = Module::new(&store, bytes).unwrap();

        let memory_initial_limits = WasmInstance::get_memory_initial_limits(bytes)?;
        
        Ok(CompiledWasmModule {
            module: wasmer_module,
            memory_initial_limits,
            store,
        })
    }
}
