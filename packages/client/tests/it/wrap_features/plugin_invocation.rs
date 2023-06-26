use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_core::{
    client::ClientConfig, error::Error, macros::uri, package::WrapPackage, uri::Uri,
};
use polywrap_msgpack::msgpack;
use polywrap_resolvers::static_resolver::{StaticResolver, StaticResolverLike};
use polywrap_tests_utils::mocks::{ArgsSetData, MemoryStoragePlugin, PluginEnv};
use std::{collections::HashMap, sync::Arc};

use polywrap_plugin::{error::PluginError, package::PluginPackage};

#[test]
fn invoke_with_env() {
    let plugin = PluginEnv {};
    let package: PluginPackage = plugin.into();
    let module = Arc::new(package) as Arc<dyn WrapPackage>;

    let plugin_static_like = StaticResolverLike::Package(uri!("ens/env-plugin.eth"), module);
    let static_resolver = StaticResolver::from(vec![plugin_static_like]);

    let env_val = msgpack!({"foo": "bar"});
    let envs = HashMap::from([(uri!("ens/env-plugin.eth"), env_val)]);
    let client = PolywrapClient::new(ClientConfig {
        envs: Some(envs),
        interfaces: None,
        resolver: Arc::new(static_resolver),
    });

    let invoke_result = client
        .invoke::<bool>(
            &uri!("ens/env-plugin.eth"),
            "checkEnvIsBar",
            Some(&msgpack!({"key": "foo"})),
            None,
            None,
        )
        .unwrap();

    assert!(invoke_result);
}

#[test]
fn invoke_methods() {
    let plugin_uri = uri!("mock/plugin");

    let mut config = PolywrapClientConfig::new();
    config.add_package(
        plugin_uri.clone(),
        Arc::new(PluginPackage::from(MemoryStoragePlugin { value: 1 })),
    );

    let client = PolywrapClient::new(config.into());

    let result = client
        .invoke::<i32>(&plugin_uri, "getData", None, None, None)
        .unwrap();
    assert_eq!(result, 1);

    let result = client
        .invoke::<bool>(
            &plugin_uri,
            "setData",
            Some(&polywrap_msgpack::serialize(&ArgsSetData { value: 42 }).unwrap()),
            None,
            None,
        )
        .unwrap();
    assert_eq!(result, true);

    let result = client
        .invoke::<i32>(&plugin_uri, "getData", None, None, None)
        .unwrap();
    assert_eq!(result, 42);
}

#[test]
fn invoke_non_existent_method_should_err() {
    let plugin_uri = uri!("mock/plugin");
    let method = String::from("iDontExist");

    let mut config = PolywrapClientConfig::new();
    config.add_package(
        plugin_uri.clone(),
        Arc::new(PluginPackage::from(MemoryStoragePlugin { value: 1 })),
    );

    let client = PolywrapClient::new(config.into());

    let result = client.invoke::<i32>(&plugin_uri, &method, None, None, None);

    if let Err(err) = result {
        assert_eq!(
            err.to_string(),
            Error::InvokeError(
                plugin_uri.to_string(),
                method.clone(),
                Error::from(PluginError::InvocationError {
                    exception: PluginError::MethodNotFoundError(method).to_string()
                })
                .to_string()
            )
            .to_string()
        );
    } else {
        panic!("Expected error, got result: {:?}", result);
    }
}
