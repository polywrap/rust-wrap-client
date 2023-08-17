extern crate polywrap;
extern crate polywrap_logger_plugin;
extern crate serde;

use std::sync::Arc;

use polywrap::*;
use polywrap_logger_plugin::LoggerPlugin;
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

    config
        .add_package(
            uri!("wrapscan.io/polywrap/logger@1.0"),
            Arc::new(logger_package),
        )
        .add_interface_implementation(
            uri!("wrapscan.io/polywrap/logger@1.0"),
            uri!("wrapscan.io/polywrap/logger@1.0"),
        )
        .add(SystemClientConfig::default().into());
    let client = PolywrapClient::new(config.build());
    let result = client.invoke::<bool>(
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
