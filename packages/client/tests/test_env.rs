use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    client::{UriRedirect, ClientConfig},
    env::Envs,
    invoke::{InvokeArgs, Invoker},
    uri::Uri,
};
use polywrap_msgpack::{decode, msgpack};

use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::resolvers::{
    static_resolver::{StaticResolver, StaticResolverLike},
};
use polywrap_resolvers::legacy::{filesystem::FilesystemResolver, base::BaseResolver};
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Deserialize;
use std::{sync::Arc, collections::HashMap};

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

#[tokio::test]
async fn test_env() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let env_wrapper: Uri = format!("fs/{}/env-type/01-main/implementations/as", path)
        .try_into()
        .unwrap();
    let as_env_external_wrapper_path: Uri =
        format!("fs/{}/env-type/00-external/implementations/as", path)
            .try_into()
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

    let redirect = UriRedirect::new(
        "ens/external-env.polywrap.eth".try_into().unwrap(),
        as_env_external_wrapper_path.clone(),
    );

    let redirects_static_like = StaticResolverLike::Redirect(redirect);
    let static_resolver = StaticResolver::from(vec![redirects_static_like]);

    let file_reader = SimpleFileReader::new();
    let client = PolywrapClient::new(ClientConfig {
        resolver: Arc::new(BaseResolver::new(
            Box::new(FilesystemResolver::new(Arc::new(file_reader))),
            Box::new(static_resolver),
        )),
        envs: Some(envs),
        interfaces: None
    });
    let invoke_args = InvokeArgs::Msgpack(msgpack!({"arg": "test"}));

    let invoke_result: Vec<u8> = client
        .invoke(
            &env_wrapper,
            "methodRequireEnv",
            Some(&invoke_args),
            None,
            None,
        )
        .await
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

    assert_eq!(
        decode::<Response>(&invoke_result as &[u8]).unwrap() as Response,
        decoded_response
    );
}
