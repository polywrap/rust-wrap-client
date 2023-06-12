use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_core::{
    uri::Uri,
    client::ClientConfig,
    builder::BuilderConfig,
    resolvers::{StaticResolverLike, RecursiveResolver, resolver_vec}
};
use polywrap_msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path_string;
use polywrap_benchmarks::load_wrapper::prepare_uris;

fn bench_load_wrapper(c: &mut Criterion) {
    let client = prepare_client();
    let uri_list = prepare_uris();

    let mut group = c.benchmark_group("client/load_wrapper");

    for uri in uri_list {
        group.bench_function(&uri.id, |b| b.iter(|| {
            let result = client
                .invoke::<u32>(
                    &uri.uri,
                    "add",
                    Some(&msgpack!({"a": 1, "b": 1})),
                    None,
                    None,
                )
                .unwrap();

            assert_eq!(result, 2);
        }));
    }
}


criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = bench_load_wrapper
}
criterion_main!(benches);