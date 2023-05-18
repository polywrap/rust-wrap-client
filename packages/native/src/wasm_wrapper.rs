use std::sync::{Arc};

use polywrap_client::core::{file_reader::SimpleFileReader, wrapper::Wrapper};
use polywrap_wasm::wasm_wrapper::WasmWrapper;

use crate::{uri::FFIUri, invoker::FFIInvoker};

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
      uri: Arc<FFIUri>,
      method: &str,
      args: Option<Vec<u8>>,
      invoker: Arc<FFIInvoker>,
      env: Option<Vec<u8>>
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
      self.inner_wasm_wrapper.invoke(invoker, &uri.0, method, args.as_deref(), env.as_deref(), None)
    }
}
