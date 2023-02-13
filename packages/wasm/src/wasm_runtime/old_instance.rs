use std::sync::{Arc, Mutex};
use polywrap_core::invoke::{Invoker};
use wasmtime::{
    AsContextMut, Config, Engine, Extern, Instance, Memory, MemoryType, Module, Store, Val,
};

use super::old_imports::create_imports;
use crate::error::WrapperError;
use crate::utils::index_of_array;

pub struct WasmInstance {
    instance: Instance,
    pub store: Store<State>,
    pub module: Module,
}

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

impl WasmInstance {
    pub async fn new(wasm_module: &Vec<u8>, shared_state: State) -> Result<Self, WrapperError> {
        let mut config = Config::new();
        config.async_support(true);

        let engine =
            Engine::new(&config).map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;
        let mut linker = wasmtime::Linker::new(&engine);

        let mut store = Store::new(&engine, shared_state);

        let module =  Module::from_binary(&engine, wasm_module).unwrap();

        let module_bytes = module
            .serialize()
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let memory = WasmInstance::create_memory(module_bytes.as_ref(), &mut store)?;

        create_imports(&mut linker, Arc::new(Mutex::new(memory)))?;

        let instance = linker
            .instantiate_async(store.as_context_mut(), &module)
            .await
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        Ok(Self {
            module,
            instance,
            store,
        })
    }

    pub async fn call_export(
        &mut self,
        name: &str,
        params: &[Val],
        results: &mut [Val],
    ) -> Result<(), WrapperError> {
        let export = self.instance.get_export(self.store.as_context_mut(), name);

        if export.is_none() {
            return Err(WrapperError::WasmRuntimeError(format!(
                "Export {} not found",
                name
            )));
        }

        match export.unwrap() {
            Extern::Func(func) => {
                func.call_async(self.store.as_context_mut(), params, results)
                    .await
                    .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

                Ok(())
            }
            _ => panic!("Export is not a function"),
        }
    }

    fn create_memory<T>(module_bytes: &[u8], store: &mut Store<T>) -> Result<Memory, WrapperError> {
        const ENV_MEMORY_IMPORTS_SIGNATURE: [u8; 11] = [
            0x65, 0x6e, 0x76, 0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x02,
        ];

        let sig_idx = index_of_array(module_bytes, &ENV_MEMORY_IMPORTS_SIGNATURE);

        if sig_idx.is_none() {
            return Err(WrapperError::ModuleReadError(
                r#"Unable to find Wasm memory import section.
            Modules must import memory from the "env" module's
            "memory" field like so:
            (import "env" "memory" (memory (;0;) #))"#
                    .to_string(),
            ));
        }

        // let memory_initial_limits =
        //     module_bytes[sig_idx.unwrap() + ENV_MEMORY_IMPORTS_SIGNATURE.len() + 1];
        let memory_type = MemoryType::new(1, Option::None);

        Memory::new(store.as_context_mut(), memory_type)
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))
    }
}
