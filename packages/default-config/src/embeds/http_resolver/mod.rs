use polywrap_core::file_reader::SimpleFileReader;
use polywrap_wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper};
use std::sync::Arc;

const WRAP_INFO: &[u8] = include_bytes!("./wrap.info");
const WRAP_WASM: &[u8] = include_bytes!("./wrap.wasm");

pub fn wasm_package() -> WasmPackage {
    WasmPackage::new(
        Arc::new(SimpleFileReader::new()),
        Some(WRAP_INFO.to_vec()),
        Some(WRAP_WASM.to_vec()),
    )
}

pub fn wasm_wrapper() -> WasmWrapper {
    WasmWrapper::new(WRAP_WASM.to_vec(), Arc::new(SimpleFileReader::new()))
}