use std::sync::{Arc, Mutex};

use polywrap_client::core::{file_reader::SimpleFileReader, wrapper::Wrapper};
use polywrap_wasm::wasm_wrapper::WasmWrapper;

pub struct FFIWasmWrapper {
    pub inner_wasm_wrapper: Arc<Mutex<Box<dyn Wrapper>>>,
}

impl FFIWasmWrapper {
    pub fn new(wasm_module: Vec<u8>) -> FFIWasmWrapper {
        let wasm_wrapper = WasmWrapper::new(wasm_module, Arc::new(SimpleFileReader::new()));
        FFIWasmWrapper {
            inner_wasm_wrapper: Arc::new(Mutex::new(Box::new(wasm_wrapper) as Box<dyn Wrapper>)),
        }
    }
}
