use polywrap_client::client::PolywrapClient;
use polywrap_client::core::interface_implementation::InterfaceImplementations;

use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::macros::uri;
use polywrap_core::uri::Uri;
use polywrap_msgpack_serde::to_vec;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ModuleMethodResponse {
    uint8: i8,
    str: String,
}

#[derive(Serialize)]
struct Args {
    arg: ModuleMethodResponse,
}

#[derive(Serialize)]
struct AbstractMethodArgs {
    arg: Object,
}

#[derive(Serialize)]
struct Object {
    str: String,
}

#[test]
fn test_interface_implementation() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let implementation_uri =
        format!("fs/{path}/interface-invoke/01-implementation/implementations/as")
            .parse()
            .unwrap();
    let wrapper_uri = format!("fs/{path}/interface-invoke/02-wrapper/implementations/as")
        .parse()
        .unwrap();

    let mut interfaces: InterfaceImplementations = HashMap::new();
    interfaces.insert(uri!("authority/interface"), vec![implementation_uri]);

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));
    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(HashMap::new())),
    );
    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: Some(interfaces),
        resolver: Arc::new(base_resolver),
    });

    let mock_response = ModuleMethodResponse {
        uint8: 1,
        str: "Test String 1".to_string(),
    };
    let invoke_result = client
        .invoke::<ModuleMethodResponse>(
            &wrapper_uri,
            "moduleMethod",
            Some(
                &to_vec(&Args {
                    arg: mock_response.clone(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();
    assert_eq!(invoke_result, mock_response);

    let abstract_method_result = client
        .invoke::<String>(
            &wrapper_uri,
            "abstractModuleMethod",
            Some(
                &to_vec(&AbstractMethodArgs {
                    arg: Object {
                        str: "test".to_string(),
                    },
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(abstract_method_result, "test".to_string())
}
