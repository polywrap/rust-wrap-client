extern crate polywrap;
extern crate polywrap_logger_plugin;
extern crate serde;

use polywrap::*;
use serde::Serialize;

#[derive(Serialize)]
struct LogMessageArgs {
    message: String,
}

fn main() {
    let wrap_uri = uri!("wrapscan.io/polywrap/logging@1.0.0");
    let mut config = PolywrapClientConfig::new();

    config.add(SystemClientConfig::default().into());
    let client = Client::new(config.build());
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
