use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::{client::ClientConfigBuilder, error::Error, macros::uri, uri::Uri};
use polywrap_msgpack_serde::to_vec;
use serde::Serialize;

#[derive(Serialize)]
struct LogMessageArgs {
    message: String,
}

fn main() {
    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());

    let uri = uri!("ipfs/Qmd3B3UPXoJYCWMjdnKa7Hs8SXpxLo2tQJfMdqpECbki7J");
    let client = PolywrapClient::new(config.build());
    let result: Result<bool, Error> = client.invoke(
        &uri,
        "logMessage",
        Some(&to_vec(&LogMessageArgs {
            message: "hey".to_string(),
        }).unwrap()),
        None,
        None,
    );

    println!("{:#?}", result);
}
