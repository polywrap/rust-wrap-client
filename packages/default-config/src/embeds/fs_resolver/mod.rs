use polywrap_core::file_reader::SimpleFileReader;
use polywrap_wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper};
use std::sync::Arc;

fn wrap_wasm() -> Vec<u8> {
    include_bytes!("./wrap.wasm").to_vec()
}

fn wrap_info() -> Vec<u8> {
    include_bytes!("./wrap.info").to_vec()
}

pub fn wasm_package() -> WasmPackage {
    WasmPackage::new(
        Arc::new(SimpleFileReader::new()),
        Some(wrap_info()),
        Some(wrap_wasm()),
    )
}

pub fn wasm_wrapper() -> WasmWrapper {
    WasmWrapper::new(wrap_wasm(), Arc::new(SimpleFileReader::new()))
}
