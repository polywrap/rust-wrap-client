use criterion::{criterion_group, criterion_main, Criterion};
use criterion::BatchSize::PerIteration;

use polywrap_client::client::PolywrapClient;
use polywrap_core::{
    uri::Uri,
    wrap_loader::WrapLoader,
};
use polywrap_client_builder::types::{ClientConfigHandler};
use polywrap_core::uri_resolver_handler::UriResolverHandler;
use polywrap_benchmarks::get_tests_path_string;

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
    let path = get_tests_path_string();
    let fs_uri = UriCase {
        id: "fs_uri".to_string(),
        uri: Uri::try_from(format!("fs/{path}/subinvoke/00-subinvoke/implementations/rs")).unwrap(),
    };
    let http_uri = UriCase {
        id: "http_uri".to_string(),
        uri: Uri::try_from(format!("http/https://raw.githubusercontent.com/polywrap/wrap-test-harness/master/cases/subinvoke/00-subinvoke/implementations/rs")).unwrap(),
    };
    let ipfs_uri = UriCase {
        id: "ipfs_uri".to_string(),
        uri: Uri::try_from("/ipfs/Qmf7jukQhTQekdSgKfdnFtB6ERTN6V7aT4oYpzesDyr2cS").unwrap(),
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

criterion_group!(benches, bench_try_resolve_uri, bench_load_wrapper);
criterion_main!(benches);