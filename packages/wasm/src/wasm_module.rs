use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use wasmer::{CompileError, Module, Store};

use crate::{
    error::WrapperError,
    runtime::instance::{State, WasmInstance},
};

#[derive(Clone)]
pub enum WasmModule {
    WasmBytecode(Vec<u8>),
    Serialized {
        compiled_bytes: Bytes,
        memory_initial_limits: u8,
    },
    Compiled(CompiledWasmModule),
}

impl WasmModule {
    pub fn compile(self) -> Result<CompiledWasmModule, WrapperError> {
        Ok(match self {
            WasmModule::WasmBytecode(bytes) => CompiledWasmModule::try_from_bytecode(&bytes)?,
            WasmModule::Serialized {
                compiled_bytes,
                memory_initial_limits,
            } => {
                let store = Store::default();
                let wasmer_module = Module::deserialize_checked(&store, compiled_bytes)?;

                CompiledWasmModule {
                    module: wasmer_module,
                    memory_initial_limits,
                    store: Arc::new(store),
                }
            }
            WasmModule::Compiled(compiled_module) => compiled_module,
        })
    }
}

#[derive(Clone)]
pub struct CompiledWasmModule {
    pub module: Module,
    pub memory_initial_limits: u8,
    pub store: Arc<Store>,
}

impl CompiledWasmModule {
    pub fn create_instance(&self, state: Arc<Mutex<State>>) -> Result<WasmInstance, WrapperError> {
        let instance = WasmInstance::new(&self.module, self.memory_initial_limits, state)?;
        Ok(instance)
    }

    pub fn try_from_bytecode(bytes: &[u8]) -> Result<Self, WrapperError> {
        let store = Store::default();
        let wasmer_module = Module::new(&store, bytes)?;

        let memory_initial_limits = WasmInstance::get_memory_initial_limits(bytes)?;

        Ok(CompiledWasmModule {
            module: wasmer_module,
            memory_initial_limits,
            store: Arc::new(store),
        })
    }
}

#[derive(thiserror::Error, Debug)]
struct CompilationError(String);

impl Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"CompilationError("{}")"#, self.0)
    }
}

impl From<CompileError> for CompilationError {
    fn from(error: CompileError) -> Self {
        Self(error.to_string())
    }
}
