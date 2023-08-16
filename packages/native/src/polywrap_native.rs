use std::sync::Arc;

use polywrap_client::core::{
    file_reader::SimpleFileReader, package::WrapPackage, wrapper::Wrapper,
};
use polywrap_wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper};

use crate::{error::FFIError, package::FFIWrapPackage, wrapper::FFIWrapper};

pub fn ffi_wrap_package_from_bytecode(bytes: &[u8]) -> Result<Arc<FFIWrapPackage>, FFIError> {
    let wasm_package =
        WasmPackage::from_bytecode(bytes.to_vec(), Arc::new(SimpleFileReader::new()), None);

    let wrap_package: Arc<dyn WrapPackage> = Arc::new(wasm_package);
    Ok(Arc::new(FFIWrapPackage(Box::new(wrap_package))))
}

pub fn ffi_wrapper_from_bytecode(bytes: &[u8]) -> Result<Arc<FFIWrapper>, FFIError> {
    let wasm_wrapper = WasmWrapper::try_from_bytecode(bytes, Arc::new(SimpleFileReader::new()))?;

    let wrapper: Arc<dyn Wrapper> = Arc::new(wasm_wrapper);

    Ok(Arc::new(FFIWrapper(Box::new(wrapper))))
}

#[cfg(test)]
mod test {
    use polywrap_tests_utils::helpers::get_tests_path;
    use std::{fs, path::Path};

    use crate::polywrap_native::ffi_wrap_package_from_bytecode;

    use super::ffi_wrapper_from_bytecode;

    #[test]
    fn test_ffi_wrapper_and_wrap_package_from_bytecode() {
        let test_path = get_tests_path().unwrap();
        let path = test_path.into_os_string().into_string().unwrap();

        let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");

        let module_bytes = fs::read(Path::new(&module_path)).unwrap();
        assert!(ffi_wrapper_from_bytecode(&module_bytes).is_ok());
        assert!(ffi_wrap_package_from_bytecode(&module_bytes).is_ok());
    }
}
