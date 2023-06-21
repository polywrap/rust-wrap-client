use polywrap_core::file_reader::SimpleFileReader;
use polywrap_wasm::{
    wasm_module::CompiledWasmModule, wasm_package::WasmPackage, wasm_wrapper::WasmWrapper,
};
use std::sync::Arc;

const WRAP_INFO: &[u8] = include_bytes!("./wrap.info");
const WRAP_WASM: &[u8] = include_bytes!("./wrap.wasm");

pub fn wasm_package() -> WasmPackage {
    WasmPackage::from_byte_code(
        WRAP_WASM.to_vec(),
        Arc::new(SimpleFileReader::new()),
        Some(WRAP_INFO.to_vec()),
    )
}

pub fn wasm_wrapper() -> WasmWrapper {
    let compiled_module = CompiledWasmModule::from_byte_code(WRAP_WASM).unwrap();
    WasmWrapper::new(compiled_module, Arc::new(SimpleFileReader::new()))
}
