use std::sync::Arc;

use polywrap_client::core::{file_reader::SimpleFileReader, package::WrapPackage};
use polywrap_wasm::wasm_package::WasmPackage;

use crate::{
    error::FFIError,
    wasm_wrapper::{FFICompiledWasmModule, FFISerializedWasmModule, FFIWasmWrapper},
};

pub struct FFIWasmPackage(Arc<WasmPackage>);

pub fn ffi_wasm_package_from_bytecode(bytes: &[u8]) -> Result<Arc<FFIWasmPackage>, FFIError> {
    let wasm_package =
        WasmPackage::from_bytecode(bytes.to_vec(), Arc::new(SimpleFileReader::new()), None);

    Ok(Arc::new(FFIWasmPackage(Arc::new(wasm_package))))
}

pub fn ffi_wasm_package_from_serialized_module(
    serialized_module: Arc<FFISerializedWasmModule>,
    bytes: &[u8],
) -> Result<Arc<FFIWasmPackage>, FFIError> {
    let wasm_package = WasmPackage::from_wasm_module(
        polywrap_wasm::wasm_module::WasmModule::Serialized(serialized_module.0.as_ref().clone()),
        bytes.to_vec(),
        Arc::new(SimpleFileReader::new()),
        None,
    )?;

    Ok(Arc::new(FFIWasmPackage(Arc::new(wasm_package))))
}

pub fn ffi_wasm_package_from_compiled_wasm_module(
    compiled_module: Arc<FFICompiledWasmModule>,
    bytes: &[u8],
) -> Result<Arc<FFIWasmPackage>, FFIError> {
    let wasm_package = WasmPackage::from_compiled_module(
        compiled_module.0.as_ref().clone(),
        bytes.to_vec(),
        Arc::new(SimpleFileReader::new()),
        None,
    )?;

    Ok(Arc::new(FFIWasmPackage(Arc::new(wasm_package))))
}

impl FFIWasmPackage {
    pub fn create_wrapper(&self) -> Result<Arc<FFIWasmWrapper>, FFIError> {
        let wrapper = self.0.create_wrapper()?;
        let wasm_wrapper = FFIWasmWrapper(wrapper);

        Ok(Arc::new(wasm_wrapper))
    }
}
