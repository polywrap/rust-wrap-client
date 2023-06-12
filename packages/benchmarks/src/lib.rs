use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use polywrap_core::error::Error;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::interface_implementation::InterfaceImplementations;
use polywrap_core::invoker::Invoker;
use polywrap_core::resolution::uri_resolution_context::UriResolutionContext;
use polywrap_core::uri::Uri;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_wrapper::WasmWrapper;

pub fn get_tests_path_string() -> String {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    path
}

pub fn get_fibonacci_wrap(implementation: &str) -> WasmWrapper {
    let relative_path = format!("fibonacci/{}/build/wrap.wasm", implementation);
    let path = Path::new(&relative_path)
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let module_bytes = fs::read(Path::new(&path)).unwrap();
    let file_reader = SimpleFileReader::new();
    let wrapper = WasmWrapper::new(module_bytes, Arc::new(file_reader));
    wrapper
}
