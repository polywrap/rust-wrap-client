use polywrap_client::client::PolywrapClient;
use polywrap_client::core::{env::Envs, uri::Uri};
use polywrap_client::msgpack::msgpack;

use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::resolvers::static_resolver::StaticResolver;
use polywrap_core::resolvers::uri_resolution_context::UriPackageOrWrapper;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, PartialEq)]
struct Response {
    str: String,
    optStr: Option<String>,
    optFilledStr: Option<String>,
    number: i8,
    optNumber: Option<i8>,
    bool: bool,
    optBool: Option<bool>,
    en: i8,
    optEnum: Option<i8>,
    object: HashMap<String, String>,
    optObject: Option<HashMap<String, String>>,
    array: Vec<i32>,
}

#[test]
fn test_env() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let env_wrapper =
        Uri::try_from(format!("fs/{path}/env-type/01-main/implementations/as")).unwrap();
    let as_env_external_wrapper_path = Uri::try_from(format!(
        "fs/{path}/env-type/00-external/implementations/as"
    ))
    .unwrap();

    let mut envs: Envs = HashMap::new();
    let external_env = json!({
        "externalArray": [1, 2, 3],
        "externalString": "iamexternal"
    });

    envs.insert(as_env_external_wrapper_path.clone().uri, external_env);

    let response = json!({
        "object": {
            "prop": "object string",
        },
        "str": "string",
        "optFilledStr": "optional string",
        "number": 10,
        "bool": true,
        "en": "FIRST",
        "array": [32, 23],
    });
    let mut obj = HashMap::new();
    obj.insert("prop".to_string(), "object string".to_string());
    envs.insert(env_wrapper.clone().uri, response);

    let mut resolvers = HashMap::new();
    resolvers.insert(
        Uri::try_from("ens/external-env.polywrap.eth").unwrap().uri,
        UriPackageOrWrapper::Uri(as_env_external_wrapper_path),
    );

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(resolvers)),
    );
    let config = ClientConfig {
        envs: Some(envs),
        resolver: Arc::new(base_resolver),
        interfaces: None,
    };
    let client = PolywrapClient::new(config);

    let invoke_result = client
        .invoke::<Response>(
            &env_wrapper,
            "methodRequireEnv",
            Some(&msgpack!({"arg": "test"})),
            None,
            None,
        )
        .unwrap();

    let decoded_response = Response {
        str: "string".to_string(),
        optStr: None,
        optFilledStr: Some("optional string".to_string()),
        number: 10,
        optNumber: None,
        bool: true,
        optBool: None,
        en: 0,
        optEnum: None,
        object: obj,
        optObject: None,
        array: vec![32, 23],
    };

    assert_eq!(invoke_result, decoded_response);
}
