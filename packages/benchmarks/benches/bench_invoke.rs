use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::resolution::uri_resolution_context::UriPackageOrWrapper;
use polywrap_msgpack::msgpack;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path_string;

fn prepare_client() -> PolywrapClient {
    let path = get_tests_path_string();
    let subinvoke_uri = Uri::try_from(format!(
        "fs/{path}/subinvoke/00-subinvoke/implementations/rs"
    )).unwrap();

    let mut resolvers = HashMap::new();
    resolvers.insert(
        String::from("wrap://ens/imported-subinvoke.eth"),
        UriPackageOrWrapper::Uri(subinvoke_uri),
    );
    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(resolvers)),
    );

    let config = ClientConfig {
        resolver: Arc::new(base_resolver),
        envs: None,
        interfaces: None,
    };
    let client = PolywrapClient::new(config);
    client
}

fn bench_invoke(c: &mut Criterion) {
    let client = prepare_client();

    // Note: this is using the correct URI for invoke, which is in the 00-subinvoke wrapper
    let path = get_tests_path_string();
    let uri = Uri::try_from(format!(
        "fs/{path}/subinvoke/00-subinvoke/implementations/rs"
    )).unwrap();

    c.bench_function("client/invoke", |b| b.iter(|| {
        let result = client
            .invoke::<u32>(
                &uri,
                "add",
                Some(&msgpack!({"a": 1, "b": 1})),
                None,
                None,
            )
            .unwrap();

        assert_eq!(result, 2);
    }));
}

fn bench_subinvoke(c: &mut Criterion) {
    let client = prepare_client();

    // Note: this is using the correct URI for subinvoke, which is in the 01-invoke wrapper
    let path = get_tests_path_string();
    let uri = Uri::try_from(format!(
        "fs/{path}/subinvoke/01-invoke/implementations/rs"
    )).unwrap();

    c.bench_function("client/subinvoke", |b| b.iter(|| {
        let result = client
            .invoke::<u32>(
                &uri,
                "addAndIncrement",
                Some(&msgpack!({"a": 1, "b": 1})),
                None,
                None,
            )
            .unwrap();

        assert_eq!(result, 3);
    }));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = bench_invoke, bench_subinvoke
}
criterion_main!(benches);
