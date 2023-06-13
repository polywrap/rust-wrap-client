use criterion::{criterion_group, criterion_main, Criterion};
use std::{path::Path};
use polywrap_wasm::{wasm_wrapper::{WasmWrapper}};
use polywrap_benchmarks::{get_fibonacci_wrap, get_tests_path_string};
use polywrap_core::{
    file_reader::{SimpleFileReader},
    wrapper::Wrapper,
};

use polywrap_msgpack::msgpack;
use std::sync::{Arc};
use std::fs;
use polywrap_tests_utils::mocks::MockInvoker;
use polywrap_benchmarks::fibonacci::{fibonacci_loop, fibonacci_recursive};

fn bench_invoke(c: &mut Criterion) {
    // Note: this is using the correct URI for invoke, which is in the 00-subinvoke wrapper
    let path = get_tests_path_string();
    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let file_reader = SimpleFileReader::new();
    let wrapper = WasmWrapper::new(module_bytes, Arc::new(file_reader));

    let mock_invoker = Arc::new(MockInvoker {});

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

fn bench_fibonacci_loop(c: &mut Criterion) {
    let mock_invoker = Arc::new(MockInvoker {});
    let n = 10_000;

    let mut group = c.benchmark_group("wasm/fibonacci_loop");

    group.bench_function("native", |b| b.iter(|| {
        fibonacci_loop(n).unwrap()
    }));

    let wrapper = get_fibonacci_wrap("as");
    group.bench_function("as", |b| b.iter(|| {
        wrapper.invoke(
            "fibonacci_loop",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));

    let wrapper = get_fibonacci_wrap("rs");
    group.bench_function("rs", |b| b.iter(|| {
        wrapper.invoke(
            "fibonacci_loop",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));

    group.finish();
}

fn bench_fibonacci_recursive(c: &mut Criterion) {
    let mock_invoker = Arc::new(MockInvoker {});

    // AS wrap crashes at higher N
    let n = 30;

    let mut group = c.benchmark_group("wasm/fibonacci_recursive");

    group.bench_function("native", |b| b.iter(|| {
        fibonacci_recursive(n).unwrap()
    }));

    let wrapper = get_fibonacci_wrap("as");
    group.bench_function("as", |b| b.iter(|| {
        wrapper.invoke(
            "fibonacci_recursive",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));

    let wrapper = get_fibonacci_wrap("rs");
    group.bench_function("rs", |b| b.iter(|| {
        wrapper.invoke(
            "fibonacci_recursive",
            Some(&msgpack!({ "n": n })),
            None,
            mock_invoker.clone(),
            None
        ).unwrap();
    }));

    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_invoke, bench_fibonacci_loop, bench_fibonacci_recursive
}
criterion_main!(benches);
