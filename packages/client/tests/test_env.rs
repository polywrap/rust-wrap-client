use polywrap_client::{polywrap_client::PolywrapClient};
use polywrap_client_builder::types::{ClientBuilder, BuilderConfig, ClientConfigHandler};
use polywrap_core::{
    env::Envs,
    uri::Uri,
};
use polywrap_msgpack::{decode, msgpack};

use polywrap_tests_utils::helpers::get_tests_path;
use serde::Deserialize;
use std::{collections::HashMap};

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
    let env_wrapper= Uri::try_from(format!("fs/{}/env-type/01-main/implementations/as", path)).unwrap();
    let as_env_external_wrapper_path = Uri::try_from(format!("fs/{}/env-type/00-external/implementations/as", path)).unwrap();

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

    let mut builder = BuilderConfig::new(None);
    builder.add_redirect(
        Uri::try_from("ens/external-env.polywrap.eth").unwrap(),
        as_env_external_wrapper_path.clone()
    );
    builder.add_envs(envs);
    let config = builder.build();
    let client = PolywrapClient::new(config);

    let invoke_result: Vec<u8> = client
        .invoke(
            &env_wrapper,
            "methodRequireEnv",
            Some(&msgpack!({"arg": "test"})),
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
