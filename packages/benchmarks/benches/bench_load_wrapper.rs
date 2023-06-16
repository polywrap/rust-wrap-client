use criterion::{criterion_group, criterion_main, Criterion};
use criterion::BatchSize::PerIteration;

use polywrap_client::client::PolywrapClient;
use polywrap_core::{
    uri::Uri,
    wrap_loader::WrapLoader,
};
use polywrap_client_builder::types::{ClientConfigHandler};
use polywrap_core::uri_resolver_handler::UriResolverHandler;
use polywrap_benchmarks::{get_fibonacci_dir};

fn prepare_client() -> PolywrapClient {
    let builder = polywrap_client_default_config::build();
    let config = builder.build();
    PolywrapClient::new(config)
}

pub struct UriCase {
    pub id: String,
    pub uri: Uri,
}

pub fn prepare_uris() -> Vec<UriCase> {
    let fs_uri = UriCase {
        id: "fs_uri".to_string(),
        uri: Uri::try_from(format!("fs/{}", get_fibonacci_dir("rs"))).unwrap(),
    };
    let http_uri = UriCase {
        id: "http_uri".to_string(),
        uri: Uri::try_from(format!(
            "http/https://raw.githubusercontent.com/polywrap/rust-client/kris/debug-slow-invocation/packages/benchmarks/fibonacci/rs/build"
        )).unwrap(),
    };
    let ipfs_uri = UriCase {
        id: "ipfs_uri".to_string(),
        uri: Uri::try_from("ipfs/QmXTYY4HAhurxZURrnRM8uD2oBCog8ATYDruVekuXQB192").unwrap(),
    };
    vec![fs_uri, http_uri, ipfs_uri]
}

fn bench_try_resolve_uri(c: &mut Criterion) {
    let uri_list = prepare_uris();

    let mut group = c.benchmark_group("client/try_resolve_uri");

    for uri in uri_list {
        group.bench_with_input(&uri.id, &uri.uri, |b, uri| b.iter_batched_ref(|| {
            prepare_client()
        }, | client: &mut PolywrapClient | {
            client.try_resolve_uri(uri, None).unwrap();
        },
      PerIteration
        ));
    }
}

fn bench_load_wrapper(c: &mut Criterion) {
    let uri_list = prepare_uris();

    let mut group = c.benchmark_group("client/load_wrapper");

    for uri in uri_list {
        group.bench_with_input(&uri.id, &uri.uri, |b, uri| b.iter_batched_ref(|| {
            prepare_client()
        }, | client: &mut PolywrapClient | {
            client.load_wrapper(uri, None).unwrap();
        },
        PerIteration
        ));
    }
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_try_resolve_uri, bench_load_wrapper
}
criterion_main!(benches);