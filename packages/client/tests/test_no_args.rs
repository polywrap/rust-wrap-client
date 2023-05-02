use std::collections::HashMap;
use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::resolvers::static_resolver::StaticResolver;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_tests_utils::helpers::get_tests_path;

#[test]
fn no_args_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let wrapper_uri = Uri::try_from(format!(
        "fs/{}/no-args/implementations/as",
        path
    ))
    .unwrap();

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(HashMap::new())),
    );

    let config = ClientConfig {
        resolver: Arc::new(base_resolver),
        envs: None,
        interfaces: None,
    };
    let client = PolywrapClient::new(config);

    let invoke_result = client
        .invoke::<bool>(
            &wrapper_uri,
            "noArgsMethod",
            None,
            None,
            None,
        )
        .unwrap();

    assert_eq!(invoke_result, true);
}
