use polywrap_core::macros::uri;
use polywrap_core::{
    error::Error, file_reader::SimpleFileReader,
    interface_implementation::InterfaceImplementations, invoker::Invoker,
    resolution::uri_resolution_context::UriResolutionContext, uri::Uri, wrapper::Wrapper,
};
use polywrap_wasm::wasm_module::CompiledWasmModule;
use polywrap_wasm::wasm_wrapper::WasmWrapper;
use serde::Serialize;
use std::{collections::HashMap, path::Path, sync::Mutex};
use wrap_manifest_schemas::deserialize::deserialize_wrap_manifest;

use polywrap_msgpack_serde::to_vec;
use polywrap_tests_utils::helpers::get_tests_path;
use std::fs;
use std::sync::Arc;

#[derive(Clone)]
struct MockInvoker {
    wrapper: WasmWrapper,
}

impl MockInvoker {
    fn new(wrapper: WasmWrapper) -> Self {
        Self { wrapper }
    }

    fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<dyn Wrapper>,
        _: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        _: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        wrapper.invoke(method, args, env, Arc::new(self.clone()))
    }
}

impl Invoker for MockInvoker {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        self.clone().invoke_wrapper_raw(
            Arc::new(self.wrapper.clone()),
            uri,
            method,
            args,
            env,
            resolution_context,
        )
    }

    fn get_implementations(&self, _uri: &Uri) -> Result<Vec<Uri>, Error> {
        Ok(vec![])
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        let i = HashMap::new();
        Some(i)
    }

    fn get_env_by_uri(&self, _: &Uri) -> Option<Vec<u8>> {
        None
    }
}

#[derive(Serialize)]
struct AddArgs {
    a: u32,
    b: u32,
}

#[test]
fn invoke_from_bytecode() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");
    let manifest_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.info");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();

    let _manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();

    let wrapper = WasmWrapper::try_from_bytecode(&module_bytes, Arc::new(file_reader)).unwrap();

    let mock_invoker = MockInvoker::new(wrapper);
    let result = Arc::new(mock_invoker)
        .invoke_raw(
            &uri!("mock/wrap"),
            "add",
            Some(&to_vec(&AddArgs { a: 1, b: 1 }).unwrap()),
            None,
            None,
        )
        .unwrap();
    assert_eq!(result, [2])
}

#[test]
fn invoke_from_compiled_module() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");
    let manifest_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.info");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();

    let _manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();

    let compiled_module = CompiledWasmModule::try_from_bytecode(&module_bytes).unwrap();
    let wrapper = WasmWrapper::new(compiled_module, Arc::new(file_reader));

    let mock_invoker = MockInvoker::new(wrapper);
    let result = Arc::new(mock_invoker)
        .invoke_raw(
            &uri!("mock/wrap"),
            "add",
            Some(&to_vec(&AddArgs { a: 1, b: 1 }).unwrap()),
            None,
            None,
        )
        .unwrap();
    assert_eq!(result, [2])
}

#[test]
fn invoke_from_deserialized_module() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");
    let manifest_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.info");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();

    let _manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();

    let compiled_module = CompiledWasmModule::try_from_bytecode(&module_bytes).unwrap();

    let result = compiled_module.serialize().unwrap();

    let module = result.deserialize().unwrap();

    let wrapper = WasmWrapper::new(module, Arc::new(file_reader));

    let mock_invoker = MockInvoker::new(wrapper);
    let result = Arc::new(mock_invoker)
        .invoke_raw(
            &uri!("mock/wrap"),
            "add",
            Some(&to_vec(&AddArgs { a: 1, b: 1 }).unwrap()),
            None,
            None,
        )
        .unwrap();
    assert_eq!(result, [2])
}
