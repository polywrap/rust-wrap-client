use criterion::{criterion_group, criterion_main, Criterion};
use std::{path::Path, collections::HashMap, sync::Mutex};
use polywrap_wasm::{wasm_wrapper::{WasmWrapper}};
use polywrap_tests_utils::helpers::get_tests_path_string;
use polywrap_core::{
    invoker::{Invoker},
    uri::Uri,
    error::Error,
    file_reader::{SimpleFileReader}, resolution::uri_resolution_context::UriResolutionContext, wrapper::Wrapper, interface_implementation::InterfaceImplementations
};

use polywrap_msgpack::msgpack;
use std::sync::{Arc};
use std::fs;

#[derive(Clone)]
struct MockInvoker {}

impl MockInvoker {
    fn new() -> Self { Self {} }
}

impl Invoker for MockInvoker {
    fn invoke_raw(
        &self,
        _uri: &Uri,
        _method: &str,
        _args: Option<&[u8]>,
        _env: Option<&[u8]>,
        _resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> { Ok(vec![]) }
    fn get_implementations(&self, _uri: &Uri) -> Result<Vec<Uri>, Error> {
        Ok(vec![])
    }
    fn get_interfaces(&self) -> Option<InterfaceImplementations> { Some(HashMap::new()) }
    fn get_env_by_uri(&self, _: &Uri) -> Option<Vec<u8>> {
        None
    }
}

fn bench_invoke(c: &mut Criterion) {
    // Note: this is using the correct URI for invoke, which is in the 00-subinvoke wrapper
    let path = get_tests_path_string();
    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let file_reader = SimpleFileReader::new();
    let wrapper = WasmWrapper::new(module_bytes, Arc::new(file_reader));

    let mock_invoker = Arc::new(MockInvoker::new());

    c.bench_function("wasm/invoke", |b| b.iter(|| {
        let result = wrapper.invoke(
            "add",
            Some(&msgpack!({ "a": 1, "b": 1})),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
        assert_eq!(result, [2]);
    }));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = bench_invoke
}
criterion_main!(benches);
