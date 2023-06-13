pub mod fibonacci;

use std::fs;
use std::path::Path;
use std::sync::{Arc};
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_wrapper::WasmWrapper;

pub fn get_tests_path_string() -> String {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    path
}

pub fn get_fibonacci_dir(implementation: &str) -> String {
    let relative_path = format!("fibonacci/{}/build", implementation);
    Path::new(&relative_path)
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn get_fibonacci_wrap(implementation: &str) -> WasmWrapper {
    let path = Path::new(&get_fibonacci_dir(implementation)).join("wrap.wasm");
    let module_bytes = fs::read(path).unwrap();
    let file_reader = SimpleFileReader::new();
    let wrapper = WasmWrapper::new(module_bytes, Arc::new(file_reader));
    wrapper
}
