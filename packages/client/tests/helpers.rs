use std::{fs, path::Path, sync::Arc};

use polywrap_core::file_reader::SimpleFileReader;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_wrapper::WasmWrapper;
use wrap_manifest_schemas::deserialize::deserialize_wrap_manifest;

pub fn get_mock_wrapper() -> WasmWrapper {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let wrapper_local_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);

    let module_path = format!("{}/wrap.wasm", wrapper_local_path);
    let manifest_path = format!("{}/wrap.info", wrapper_local_path);

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();

    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();

    WasmWrapper::new(
        module_bytes,
        Arc::new(file_reader),
        manifest
    )
}