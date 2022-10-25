use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use polywrap_core::invoke::Invoker;
use tokio::runtime::Handle;
use wasmtime::{
    AsContextMut, Config, Engine, Extern, Instance, Memory, MemoryType, Module, Store, Val,
};

use crate::error::WrapperError;
use super::imports::create_imports;
use crate::utils::index_of_array;

pub struct WasmInstance {
    instance: Instance,
    pub shared_state: Arc<Mutex<State>>,
    store: Store<u32>,
    pub module: Module,
}

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

#[derive(Default)]
pub struct State {
    pub method: Vec<u8>,
    pub args: Vec<u8>,
    pub invoke: InvokeState,
    pub subinvoke: InvokeState,
}

impl State {
    pub fn new(method: &str, args: Vec<u8>) -> Self {
        Self {
            method: method.as_bytes().to_vec(),
            args,
            invoke: InvokeState::default(),
            subinvoke: InvokeState::default(),
        }
    }
}

impl WasmInstance {
    pub async fn new(
        wasm_module: &WasmModule,
        shared_state: Arc<Mutex<State>>,
        abort: Arc<dyn Fn(String) + Send + Sync>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Self, WrapperError> {
        let mut config = Config::new();
        config.async_support(true);

        let runtime = Handle::current();

        let rt = Arc::new(runtime);

        let engine =
            Engine::new(&config).map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;
        let mut linker = wasmtime::Linker::new(&engine);

        let mut store = Store::new(&engine, 4);
        let module_result = match wasm_module {
            WasmModule::Bytes(ref bytes) => Module::new(&engine, bytes),
            WasmModule::Wat(ref wat) => Module::new(&engine, wat),
            WasmModule::Path(ref path) => Module::from_file(&engine, path),
        };

        let module = module_result.map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;
        let module_bytes = module
            .serialize()
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        let memory = Rc::new(RefCell::new(WasmInstance::create_memory(
            module_bytes.as_ref(),
            &mut store,
        )?));

        create_imports(
            &mut linker,
            Arc::clone(&shared_state),
            abort,
            memory,
            invoker,
        )?;

        let instance = linker.instantiate_async(store.as_context_mut(), &module).await
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

        Ok(Self {
            module,
            shared_state,
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
              func.call_async(self.store.as_context_mut(), params, results).await
                    .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))?;

                Ok(())
            }
            _ => panic!("Export is not a function"),
        }
    }

    fn create_memory(module_bytes: &[u8], store: &mut Store<u32>) -> Result<Memory, WrapperError> {
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

        let memory_initial_limits =
            module_bytes[sig_idx.unwrap() + ENV_MEMORY_IMPORTS_SIGNATURE.len() + 1];
        let memory_type = MemoryType::new(memory_initial_limits.into(), Option::None);

        Memory::new(store.as_context_mut(), memory_type)
            .map_err(|e| WrapperError::WasmRuntimeError(e.to_string()))
    }
}
