use std::collections::HashMap;
use std::sync::Arc;

use polywrap_client::client::Client;
use polywrap_client::core::uri::Uri;
use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::macros::uri;
use polywrap_core::resolution::uri_resolution_context::UriPackageOrWrapper;
use polywrap_msgpack_serde::to_vec;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Serialize;

#[derive(Serialize)]
struct AddArgs {
    a: u32,
    b: u32,
}

#[test]
fn subinvoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let invoke_uri = format!("fs/{path}/subinvoke/01-invoke/implementations/rs")
        .parse()
        .unwrap();
    let subinvoke_uri = format!("fs/{path}/subinvoke/00-subinvoke/implementations/rs")
        .parse()
        .unwrap();

    let mut resolvers = HashMap::new();
    resolvers.insert(
        uri!("authority/imported-subinvoke"),
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
    let client = Client::new(config);

    let invoke_result = client
        .invoke::<u32>(
            &invoke_uri,
            "addAndIncrement",
            Some(&to_vec(&AddArgs { a: 1, b: 1 }).unwrap()),
            None,
            None,
        )
        .unwrap();

    assert_eq!(invoke_result, 3);
}
