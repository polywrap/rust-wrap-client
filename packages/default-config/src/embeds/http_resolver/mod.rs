use polywrap_core::file_reader::SimpleFileReader;
use polywrap_wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper, wasm_module::{SerializedWasmModule, WasmModule}};
use std::sync::Arc;

const WRAP_INFO: &[u8] = include_bytes!("./wrap.info");
const WRAP_WASM: &[u8] = include_bytes!("./wrap.wasm");
const WRAP_SERIALIZED_BYTES: &[u8] = include_bytes!("./wrap.serialized");

pub fn wasm_package() -> WasmPackage {
    let module = SerializedWasmModule::deserialize_from_storage(WRAP_SERIALIZED_BYTES);
    WasmPackage::from_wasm_module(
        WasmModule::Serialized(module),
        WRAP_WASM.to_vec(),
        Arc::new(SimpleFileReader::new()),
        Some(WRAP_INFO.to_vec()),
    )
}

pub fn wasm_wrapper() -> WasmWrapper {
    WasmWrapper::try_from_bytecode(WRAP_WASM, Arc::new(SimpleFileReader::new())).unwrap()
}
