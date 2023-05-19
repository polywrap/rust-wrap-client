use polywrap_client::client::PolywrapClient;
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;

use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::resolution::uri_resolution_context::UriPackageOrWrapper;
use polywrap_core::resolution::uri_resolver::UriResolver;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::recursive_resolver::RecursiveResolver;
use polywrap_resolvers::resolver_vec;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

fn get_env_wrapper_uri() -> Uri {
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

fn build_client(uri: &Uri, env: Option<&[u8]>) -> PolywrapClient {
    let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
   
    if let Some(env) = env {
        envs.insert(uri.to_string(), env.to_vec());
    }

    let resolvers = HashMap::new();

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
    
    PolywrapClient::new(config)
}

#[test]
fn invoke_method_without_env_does_not_require_env() {
    let wrapper_uri = get_env_wrapper_uri();

    let client = build_client(&wrapper_uri, None);

    let test_string = "test";
    let result = client
        .invoke::<String>(
            &wrapper_uri,
            "methodNoEnv",
            Some(&msgpack!({"arg": test_string})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, test_string);
}

#[test]
fn invoke_method_without_env_works_with_env() {
    let wrapper_uri = get_env_wrapper_uri();

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

    let client = build_client(&wrapper_uri, Some(&polywrap_msgpack::serialize(&env).unwrap()));

    let test_string = "test";
    let result = client
        .invoke::<String>(
            &wrapper_uri,
            "methodNoEnv",
            Some(&msgpack!({"arg": test_string})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, test_string);
}

#[test]
fn invoke_method_with_required_env_works_with_env() {
    let wrapper_uri = get_env_wrapper_uri();

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

    let client = build_client(&wrapper_uri, Some(&polywrap_msgpack::serialize(&env).unwrap()));

    let result = client
        .invoke::<Env>(
            &wrapper_uri,
            "methodRequireEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, env);
}

#[test]
#[should_panic(expected = "Environment is not set, and it is required")]
fn invoke_method_with_required_env_panics_without_env_registered() {
    let wrapper_uri = get_env_wrapper_uri();

    let client = build_client(&wrapper_uri, None);

    let result = client
        .invoke::<Option<Env>>(
            &wrapper_uri,
            "methodRequireEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, None);
}

#[test]
fn invoke_method_with_optional_env_works_with_env() {
    let wrapper_uri = get_env_wrapper_uri();

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

    let client = build_client(&wrapper_uri, Some(&polywrap_msgpack::serialize(&env).unwrap()));

    let result = client
        .invoke::<Env>(
            &wrapper_uri,
            "methodOptionalEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, env);
}

#[test]
fn invoke_method_with_optional_env_works_without_env() {
    let wrapper_uri = get_env_wrapper_uri();

    let client = build_client(&wrapper_uri, None);

    let result = client
        .invoke::<Option<Env>>(
            &wrapper_uri,
            "methodOptionalEnv",
            Some(&msgpack!({})),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, None);
}

#[test]
fn env_can_be_registered_for_any_uri_in_resolution_path() {
    let wrapper_uri = get_env_wrapper_uri();
    let redirect_from_uri = Uri::try_from("mock/from").unwrap();

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

    // Register the env for the redirect_from_uri which will be redirected to the wrapper_uri
    {
        let client = {
            let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
        
            envs.insert(redirect_from_uri.to_string(), polywrap_msgpack::serialize(&env).unwrap());
        
            let resolvers = HashMap::from([
                (
                    redirect_from_uri.to_string(),
                    UriPackageOrWrapper::Uri(wrapper_uri.clone())
                ),
            ]);
        
            let file_reader = SimpleFileReader::new();
            let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));
        
            let config = ClientConfig {
                envs: Some(envs),
                // Use the RecursiveResolver because it tracks resolution path (unlike BaseResolver)
                resolver: Arc::new(RecursiveResolver::from(resolver_vec![
                    StaticResolver::new(resolvers),
                    fs_resolver,
                ])),
                interfaces: None,
            };
            
            PolywrapClient::new(config)
        };

        let result = client
            .invoke::<Env>(
                &redirect_from_uri,
                "methodRequireEnv",
                Some(&msgpack!({})),
                None,
                None,
            )
            .unwrap();

        assert_eq!(result, env);
    }

    // Register the env for the wrapper_uri which will be redirected to, from the redirect_from_uri
    {
        let client = {
            let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
        
            envs.insert(wrapper_uri.to_string(), polywrap_msgpack::serialize(&env).unwrap());
        
            let resolvers = HashMap::from([
                (
                    redirect_from_uri.to_string(),
                    UriPackageOrWrapper::Uri(wrapper_uri)
                ),
            ]);
        
            let file_reader = SimpleFileReader::new();
            let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));
        
            let config = ClientConfig {
                envs: Some(envs),
                // Use the RecursiveResolver because it tracks resolution path (unlike BaseResolver)
                resolver: Arc::new(RecursiveResolver::from(resolver_vec![
                    StaticResolver::new(resolvers),
                    fs_resolver,
                ])),
                interfaces: None,
            };
            
            PolywrapClient::new(config)
        };

        let result = client
            .invoke::<Env>(
                &redirect_from_uri,
                "methodRequireEnv",
                Some(&msgpack!({})),
                None,
                None,
            )
            .unwrap();

        assert_eq!(result, env);
    }
}
