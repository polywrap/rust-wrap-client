use polywrap_client::client::Client;
use polywrap_client_builder::{ClientConfig, ClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_msgpack_serde::to_vec;

use crate::fs::ArgsAdd;

const SUBINVOKE_WRAP_URI: &str = "wrap://ipfs/Qmf7jukQhTQekdSgKfdnFtB6ERTN6V7aT4oYpzesDyr2cS";

#[test]
fn sanity() {
    let mut config = ClientConfig::new();
    config.add(SystemClientConfig::precompiled().into());

    let client = Client::new(config.into());
    let result = client
        .invoke::<u32>(
            &SUBINVOKE_WRAP_URI.parse().unwrap(),
            "add",
            Some(&to_vec(&ArgsAdd { a: 2, b: 40 }).unwrap()),
            None,
        )
        .unwrap();

    assert_eq!(result, 42);
}
