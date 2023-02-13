use std::{sync::{Arc, Mutex}};
use polywrap_core::invoke::{Invoker};
use wasmer::{Module, Instance, Store, Memory, MemoryType, Value, Extern};

use crate::error::WrapperError;

use super::imports::create_imports;

#[derive(Clone)]
pub enum WasmModule {
    Bytes(Vec<u8>),
    Wat(String),
    Path(String),
}

#[derive(Default)]
pub struct InvokeState {
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
}

pub struct SubinvokeImplementationState {
    pub result: Option<Vec<u8>>,
    pub error: Option<String>,
    pub args: Vec<u8>
}

pub struct State {
    pub method: Vec<u8>,
    pub args: Vec<u8>,
    pub env: Vec<u8>,
    pub invoke: InvokeState,
    pub subinvoke: InvokeState,
    pub abort: Box<dyn Fn(String) + Send + Sync>,
    pub invoker: Arc<dyn Invoker>,
    pub get_implementations_result: Option<Vec<u8>>,
    pub subinvoke_implementation: Option<SubinvokeImplementationState>
}

impl State {
    pub fn new(
        invoker: Arc<dyn Invoker>,
        abort: Box<dyn Fn(String) + Send + Sync>,
        method: &str,
        args: Vec<u8>,
        env: Vec<u8>
    ) -> Self {
        Self {
            method: method.as_bytes().to_vec(),
            args,
            env,
            invoke: InvokeState::default(),
            subinvoke: InvokeState::default(),
            abort,
            invoker,
            get_implementations_result: None,
            subinvoke_implementation: None
        }
    }
}

pub struct WasmInstance {
    instance: Instance,
    pub store: Store,
    pub module: Module,
}

impl WasmInstance {
    pub async fn new(wasm_module: &Vec<u8>, state: State) -> Result<Self, WrapperError> {
        let mut store = Store::default();
        let module = Module::new(&mut store, wasm_module.to_vec()).unwrap();
        let memory = WasmInstance::create_memory(&mut store)?;
        let (imports, mut store) = create_imports(
            Arc::new(Mutex::new(memory)),
            store,
            state
        );
        let instance = Instance::new(&mut store, &module, &imports)
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        Ok(Self {
            instance,
            store,
            module,
        })
    }

    pub fn create_memory(&mut store: &mut Store) -> Result<Memory, WrapperError> {
        let memory = Memory::new(&mut store, 
            MemoryType::new(1, None, true)
        ).unwrap();

        Ok(memory)
    }

    pub async fn call_export(
        &mut self,
        name: &str,
        params: &[Value],
        results: &mut [Value],
    ) -> Result<State, WrapperError> {
        let export = self.instance.exports.get_function(name);
        if export.is_err() {
            return Err(WrapperError::WasmRuntimeError(format!(
                "Export {} not found",
                name
            )));
        }
        let function = export.unwrap();        
        function.call(&mut self.store, params).unwrap();

        let memory = self.instance.exports.get_memory("memory").unwrap();
        let state = memory.view(&self.store);
        
        Ok(())
    }
}
