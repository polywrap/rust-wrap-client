use std::sync::{Arc};

use polywrap_client::core::{file_reader::SimpleFileReader, wrapper::Wrapper};
use polywrap_wasm::wasm_wrapper::WasmWrapper;

use crate::{wrapper::FFIAbortHandler, invoker::{FFIInvoker, FFIInvokerWrapping}};

pub struct FFIWasmWrapper {
    pub inner_wasm_wrapper: Arc<dyn Wrapper>,
}

impl FFIWasmWrapper {
    pub fn new(wasm_module: Vec<u8>) -> FFIWasmWrapper {
        let wasm_wrapper = WasmWrapper::new(wasm_module, Arc::new(SimpleFileReader::new()));
        FFIWasmWrapper {
            inner_wasm_wrapper: Arc::new(wasm_wrapper),
        }
    }

    pub fn invoke(
      &self,
      method: &str,
      args: Option<Vec<u8>>,
      env: Option<Vec<u8>>,
      invoker: Box<dyn FFIInvoker>,
      abort_handler: Option<Box<dyn FFIAbortHandler>>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
      let abort_handler = abort_handler.map(|a| 
        Box::new(move |msg: String| a.abort(msg)) as Box<dyn Fn(String) + Send + Sync>
      );

      self.inner_wasm_wrapper.invoke(method, args.as_deref(), env.as_deref(), Arc::new(FFIInvokerWrapping(invoker)), abort_handler)
    }
}
