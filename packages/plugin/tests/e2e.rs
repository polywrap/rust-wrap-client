use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_core::{macros::uri, error::Error};
use polywrap_client::core::uri::Uri;
use polywrap_plugin::{package::PluginPackage, error::PluginError};
use polywrap_tests_utils::mocks::{MemoryStoragePlugin, ArgsSetData};

#[test]
fn invoke_methods() {
    let plugin_uri = uri!("mock/plugin");

    let mut config = PolywrapClientConfig::new();
    config.add_package(plugin_uri.clone(), Arc::new(PluginPackage::from(MemoryStoragePlugin {
        value: 1
    })));

    let client = PolywrapClient::new(config.into());

    let result = client
        .invoke::<i32>(
            &plugin_uri,
            "getData",
            None,
            None,
            None,
        )
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
        .invoke::<i32>(
            &plugin_uri,
            "getData",
            None,
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, 42);
}

#[test]
fn invoke_non_existent_method_should_err() {
    let plugin_uri = uri!("mock/plugin");
    let method = String::from("iDontExist");

    let mut config = PolywrapClientConfig::new();
    config.add_package(plugin_uri.clone(), Arc::new(PluginPackage::from(MemoryStoragePlugin {
        value: 1
    })));

    let client = PolywrapClient::new(config.into());

    let result = client
        .invoke::<i32>(
            &plugin_uri,
            &method,
            None,
            None,
            None,
        );
    
    if let Err(err) = result {
        assert_eq!(
            err.to_string(), 
            Error::InvokeError(
                plugin_uri.to_string(), 
                method.clone(), 
                Error::from(PluginError::InvocationError{
                    exception: PluginError::MethodNotFoundError(method).to_string()
                }).to_string()
            ).to_string()
        );
    } else {
        panic!("Expected error, got result: {:?}", result);
    }
}
