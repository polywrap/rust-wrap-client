use polywrap_client::client::PolywrapClient;
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;

use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::resolution::uri_resolution_context::UriPackageOrWrapper;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

fn get_subinvoker_uri() -> Uri {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    

    Uri::try_from(format!(
        "fs/{path}/env-type/01-subinvoker/implementations/rs"
    ))
    .unwrap()
}

fn get_subinvoker_with_env_uri() -> Uri {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    
    

    Uri::try_from(format!(
        "fs/{path}/env-type/02-subinvoker-with-env/implementations/rs"
    ))
    .unwrap()
}

fn get_subinvoked_uri() -> Uri {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    
    

    Uri::try_from(format!(
        "fs/{path}/env-type/00-main/implementations/rs"
    ))
    .unwrap()
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Env {
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

fn build_client(subinvoker_env: Option<&[u8]>, subinvoked_env: Option<&[u8]>) -> PolywrapClient {
    let subinvoker_uri = get_subinvoker_uri();
    let subinvoked_uri = get_subinvoked_uri();

    let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
   
    if let Some(env) = subinvoker_env {
        envs.insert(subinvoker_uri.to_string(), env.to_vec());
    }

    if let Some(env) = subinvoked_env {
        envs.insert(Uri::try_from("mock/main").unwrap().to_string(), env.to_vec());
    }

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let mut resolvers = HashMap::new();
    resolvers.insert(
        Uri::try_from("mock/main").unwrap().to_string(),
        UriPackageOrWrapper::Uri(subinvoked_uri),
    );

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(resolvers)),
    );
    let config = ClientConfig {
        envs: Some(envs),
        resolver: Arc::new(base_resolver),
        interfaces: None,
    };
    
    PolywrapClient::new(config)
}

#[test]
fn subinvoke_method_without_env_does_not_require_env() {
    let subinvoker_uri = get_subinvoker_uri();

    let client = build_client(None, None);

    let test_string = "test";
    let result = client
        .invoke::<String>(
            &subinvoker_uri,
            "subinvokeMethodNoEnv",
            Some(&msgpack!({"arg": test_string})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, test_string);
}

#[test]
fn subinvoke_method_without_env_works_with_env() {
    let subinvoker_uri = get_subinvoker_uri();

    let env = Env {
        str: "string".to_string(),
        optStr: None,
        optFilledStr: Some("optional string".to_string()),
        number: 10,
        optNumber: None,
        bool: true,
        optBool: None,
        en: 0,
        optEnum: None,
        object: HashMap::from([
            ("prop".to_string(), "object string".to_string()),
        ]),
        optObject: None,
        array: vec![32, 23],
    };

    let client = build_client(None, Some(&polywrap_msgpack::serialize(&env).unwrap()));

    let test_string = "test";
    let result = client
        .invoke::<String>(
            &subinvoker_uri,
            "subinvokeMethodNoEnv",
            Some(&msgpack!({"arg": test_string})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, test_string);
}

#[test]
fn subinvoke_method_with_required_env_works_with_env() {
    let subinvoker_uri = get_subinvoker_uri();

    let env = Env {
        str: "string".to_string(),
        optStr: None,
        optFilledStr: Some("optional string".to_string()),
        number: 10,
        optNumber: None,
        bool: true,
        optBool: None,
        en: 0,
        optEnum: None,
        object: HashMap::from([
            ("prop".to_string(), "object string".to_string()),
        ]),
        optObject: None,
        array: vec![32, 23],
    };

    let client = build_client(None, Some(&polywrap_msgpack::serialize(&env).unwrap()));

    let result = client
        .invoke::<Env>(
            &subinvoker_uri,
            "subinvokeMethodRequireEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, env);
}

#[test]
#[should_panic(expected = "Environment is not set, and it is required")]
fn subinvoke_method_with_required_env_panics_without_env_registered() {
    let subinvoker_uri = get_subinvoker_uri();

    let client = build_client(None, None);

    let result = client
        .invoke::<Option<Env>>(
            &subinvoker_uri,
            "subinvokeMethodRequireEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, None);
}

#[test]
fn subinvoke_method_with_optional_env_works_with_env() {
    let subinvoker_uri = get_subinvoker_uri();

    let env = Env {
        str: "string".to_string(),
        optStr: None,
        optFilledStr: Some("optional string".to_string()),
        number: 10,
        optNumber: None,
        bool: true,
        optBool: None,
        en: 0,
        optEnum: None,
        object: HashMap::from([
            ("prop".to_string(), "object string".to_string()),
        ]),
        optObject: None,
        array: vec![32, 23],
    };

    let client = build_client(None, Some(&polywrap_msgpack::serialize(&env).unwrap()));

    let result = client
        .invoke::<Env>(
            &subinvoker_uri,
            "subinvokeMethodOptionalEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, env);
}

#[test]
fn subinvoke_method_with_optional_env_works_without_env() {
    let subinvoker_uri = get_subinvoker_uri();

    let client = build_client(None, None);

    let result = client
        .invoke::<Option<Env>>(
            &subinvoker_uri,
            "subinvokeMethodOptionalEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, None);
}

#[test]
fn subinvoker_env_does_not_override_subinvoked_env() {
    let subinvoker_uri = get_subinvoker_with_env_uri();
    let subinvoked_uri = get_subinvoked_uri();

    let subinvoker_env = Env {
        str: "string".to_string(),
        optStr: None,
        optFilledStr: Some("optional string".to_string()),
        number: 1,
        optNumber: None,
        bool: true,
        optBool: None,
        en: 0,
        optEnum: None,
        object: HashMap::from([
            ("prop".to_string(), "object string".to_string()),
        ]),
        optObject: None,
        array: vec![1, 2],
    };

    let subinvoked_env = Env {
        str: "string2".to_string(),
        optStr: None,
        optFilledStr: Some("optional string2".to_string()),
        number: 2,
        optNumber: None,
        bool: true,
        optBool: None,
        en: 0,
        optEnum: None,
        object: HashMap::from([
            ("prop".to_string(), "object string2".to_string()),
        ]),
        optObject: None,
        array: vec![2, 3],
    };

    let client = {
    
        let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
       
        envs.insert(subinvoker_uri.to_string(), polywrap_msgpack::serialize(&subinvoker_env).unwrap());
        envs.insert(Uri::try_from("mock/main").unwrap().to_string(), polywrap_msgpack::serialize(&subinvoked_env).unwrap());

        let file_reader = SimpleFileReader::new();
        let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));
    
        let mut resolvers = HashMap::new();
        resolvers.insert(
            Uri::try_from("mock/main").unwrap().to_string(),
            UriPackageOrWrapper::Uri(subinvoked_uri),
        );
    
        let base_resolver = BaseResolver::new(
            Box::new(fs_resolver),
            Box::new(StaticResolver::new(resolvers)),
        );
        let config = ClientConfig {
            envs: Some(envs),
            resolver: Arc::new(base_resolver),
            interfaces: None,
        };
        
        PolywrapClient::new(config)
    };

    let result = client
        .invoke::<Env>(
            &subinvoker_uri,
            "subinvokeMethodRequireEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, subinvoked_env);
}
