use criterion::{criterion_group, criterion_main, Criterion};
use std::{path::Path, collections::HashMap, sync::Mutex};
use polywrap_wasm::{wasm_wrapper::{WasmWrapper}};
use polywrap_benchmarks::get_tests_path_string;
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

fn get_fibonacci_wrap(implementation: &str) -> WasmWrapper {
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

fn bench_fibonacci_loop(c: &mut Criterion) {
    let mock_invoker = Arc::new(MockInvoker::new());
    let n = 42;

    let mut group = c.benchmark_group("wasm/fibonacci_loop");

    let wrapper = get_fibonacci_wrap("as");
    group.bench_function("as", |b| b.iter(|| {
        let result = wrapper.invoke(
            "fibonacci_loop",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));

    let wrapper = get_fibonacci_wrap("rs");
    group.bench_function("rs", |b| b.iter(|| {
        let result = wrapper.invoke(
            "fibonacci_loop",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));
}

fn bench_fibonacci_recursive(c: &mut Criterion) {
    let mock_invoker = Arc::new(MockInvoker::new());
    let n = 42;

    let mut group = c.benchmark_group("wasm/fibonacci_recursive");

    let wrapper = get_fibonacci_wrap("as");
    group.bench_function("as", |b| b.iter(|| {
        let result = wrapper.invoke(
            "fibonacci_recursive",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));

    let wrapper = get_fibonacci_wrap("rs");
    group.bench_function("rs", |b| b.iter(|| {
        let result = wrapper.invoke(
            "fibonacci_recursive",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));
}

criterion_group!(benches, bench_invoke, bench_fibonacci_loop, bench_fibonacci_recursive);

criterion_main!(benches);
