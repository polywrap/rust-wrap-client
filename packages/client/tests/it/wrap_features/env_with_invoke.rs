use polywrap_client::client::Client;
use polywrap_client::core::uri::Uri;

use polywrap_core::client::CoreClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::macros::uri;
use polywrap_core::resolution::uri_resolution_context::UriPackageOrWrapper;
use polywrap_core::resolution::uri_resolver::UriResolver;
use polywrap_msgpack_serde::to_vec;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::recursive_resolver::RecursiveResolver;
use polywrap_resolvers::resolver_vec;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

fn get_env_wrapper_uri() -> Uri {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    format!("fs/{path}/env-type/00-main/implementations/rs")
        .parse()
        .unwrap()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EnvObject {
    pub prop: String,
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
    object: EnvObject,
    optObject: Option<EnvObject>,
    array: Vec<i32>,
}

fn get_default_env() -> Env {
    Env {
        str: "string".to_string(),
        optStr: None,
        optFilledStr: Some("optional string".to_string()),
        number: 10,
        optNumber: None,
        bool: true,
        optBool: None,
        en: 0,
        optEnum: None,
        object: EnvObject {
            prop: "object string".to_string(),
        },
        optObject: None,
        array: vec![32, 23],
    }
}

fn get_default_serialized_env() -> Vec<u8> {
    polywrap_msgpack_serde::to_vec(&get_default_env()).unwrap()
}

fn build_client(uri: &Uri, env: Option<&[u8]>) -> Client {
    let mut envs = HashMap::new();

    if let Some(env) = env {
        envs.insert(uri.clone(), env.to_vec());
    }

    let resolvers = HashMap::new();

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(resolvers)),
    );
    let config = CoreClientConfig {
        envs: Some(envs),
        resolver: Arc::new(base_resolver),
        interfaces: None,
    };

    Client::new(config)
}

#[derive(Serialize)]
struct Args {
    arg: String,
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
            Some(
                &to_vec(&Args {
                    arg: test_string.to_string(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, test_string);
}

#[test]
fn invoke_method_without_env_works_with_env() {
    let wrapper_uri = get_env_wrapper_uri();
    let client = build_client(&wrapper_uri, Some(&get_default_serialized_env()));

    let test_string = "test";
    let result = client
        .invoke::<String>(
            &wrapper_uri,
            "methodNoEnv",
            Some(
                &to_vec(&Args {
                    arg: test_string.to_string(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, test_string);
}

#[test]
fn invoke_method_with_required_env_works_with_env() {
    let wrapper_uri = get_env_wrapper_uri();

    let client = build_client(&wrapper_uri, Some(&get_default_serialized_env()));

    let result = client
        .invoke::<Env>(&wrapper_uri, "methodRequireEnv", None, None, None)
        .unwrap();

    assert_eq!(result, get_default_env());
}

#[test]
#[should_panic(expected = "Environment is not set, and it is required")]
fn invoke_method_with_required_env_panics_without_env_registered() {
    let wrapper_uri = get_env_wrapper_uri();

    let client = build_client(&wrapper_uri, None);

    let result = client
        .invoke::<Option<Env>>(&wrapper_uri, "methodRequireEnv", None, None, None)
        .unwrap();

    assert_eq!(result, None);
}

#[test]
fn invoke_method_with_optional_env_works_with_env() {
    let wrapper_uri = get_env_wrapper_uri();

    let client = build_client(&wrapper_uri, Some(&get_default_serialized_env()));

    let result = client
        .invoke::<Env>(&wrapper_uri, "methodOptionalEnv", None, None, None)
        .unwrap();

    assert_eq!(result, get_default_env());
}

#[test]
fn invoke_method_with_optional_env_works_without_env() {
    let wrapper_uri = get_env_wrapper_uri();

    let client = build_client(&wrapper_uri, None);

    let result = client
        .invoke::<Option<Env>>(&wrapper_uri, "methodOptionalEnv", None, None, None)
        .unwrap();

    assert_eq!(result, None);
}

#[test]
fn env_can_be_registered_for_any_uri_in_resolution_path() {
    let wrapper_uri = get_env_wrapper_uri();
    let redirect_from_uri = uri!("mock/from");

    let env = get_default_env();

    // Register the env for the redirect_from_uri which will be redirected to the wrapper_uri
    {
        let client = {
            let mut envs = HashMap::new();

            envs.insert(redirect_from_uri.clone(), get_default_serialized_env());

            let resolvers = HashMap::from([(
                redirect_from_uri.clone(),
                UriPackageOrWrapper::Uri(wrapper_uri.clone()),
            )]);

            let file_reader = SimpleFileReader::new();
            let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

            let config = CoreClientConfig {
                envs: Some(envs),
                // Use the RecursiveResolver because it tracks resolution path (unlike BaseResolver)
                resolver: Arc::new(RecursiveResolver::from(resolver_vec![
                    StaticResolver::new(resolvers),
                    fs_resolver,
                ])),
                interfaces: None,
            };

            Client::new(config)
        };

        let result = client
            .invoke::<Env>(&redirect_from_uri, "methodRequireEnv", None, None, None)
            .unwrap();

        assert_eq!(result, env);
    }

    // Register the env for the wrapper_uri which will be redirected to, from the redirect_from_uri
    {
        let client = {
            let mut envs: HashMap<Uri, Vec<u8>> = HashMap::new();

            envs.insert(
                wrapper_uri.clone(),
                polywrap_msgpack_serde::to_vec(&env).unwrap(),
            );

            let resolvers = HashMap::from([(
                redirect_from_uri.clone(),
                UriPackageOrWrapper::Uri(wrapper_uri),
            )]);

            let file_reader = SimpleFileReader::new();
            let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

            let config = CoreClientConfig {
                envs: Some(envs),
                // Use the RecursiveResolver because it tracks resolution path (unlike BaseResolver)
                resolver: Arc::new(RecursiveResolver::from(resolver_vec![
                    StaticResolver::new(resolvers),
                    fs_resolver,
                ])),
                interfaces: None,
            };

            Client::new(config)
        };

        let result = client
            .invoke::<Env>(&redirect_from_uri, "methodRequireEnv", None, None, None)
            .unwrap();

        assert_eq!(result, env);
    }
}
