extern crate polywrap_client;
extern crate polywrap_client_builder;
extern crate polywrap_client_default_config;
extern crate polywrap_core;
extern crate polywrap_logger_plugin;
extern crate polywrap_msgpack_serde;
extern crate polywrap_plugin;
extern crate serde;

use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::{client::ClientConfigBuilder, error::Error, macros::uri, uri::Uri};
use polywrap_logger_plugin::LoggerPlugin;
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::package::PluginPackage;
use serde::Serialize;

#[derive(Serialize)]
struct LogMessageArgs {
    message: String,
}

fn main() {
    let wrap_uri = uri!("wrapscan.io/polywrap/logging@1.0.0");
    let mut config = PolywrapClientConfig::new();

    let logger_plugin = LoggerPlugin::new(None);
    let logger_package = PluginPackage::from(logger_plugin);

    config.add_package(
        uri!("wrapscan.io/polywrap/logger@1.0"),
        Arc::new(logger_package),
    );
    config.add_interface_implementation(
        uri!("wrapscan.io/polywrap/logger@1.0"),
        uri!("wrapscan.io/polywrap/logger@1.0"),
    );
    config.add(SystemClientConfig::default().into());
    let client = PolywrapClient::new(config.build());
    let result: Result<bool, Error> = client.invoke(
        &wrap_uri,
        "info",
        Some(
            &to_vec(&LogMessageArgs {
                message: "Hello from hello world wrap!".to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if result.is_err() {
        panic!("Error in hello world example")
    }
}
