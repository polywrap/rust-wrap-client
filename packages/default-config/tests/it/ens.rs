use polywrap_client::client::Client;
use polywrap_client_builder::{ClientConfig, ClientConfigBuilder};
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};
use polywrap_msgpack_serde::to_vec;
use serde::Serialize;

#[derive(Serialize)]
pub struct ToWeiArgs {
    pub eth: String,
}

#[test]
fn text_record_uri_resolver() {
    let wrap_uri = format!("ens/ethers.wraps.eth:utils@0.1.1");
    let mut config = ClientConfig::new();
    config
        .add(SystemClientConfig::precompiled().into())
        .add(Web3ClientConfig::default().into());

    let client = Client::new(config.into());
    let result = client.invoke::<String>(
        &wrap_uri.parse().unwrap(),
        "toWei",
        Some(
            &to_vec(&ToWeiArgs {
                eth: "20".to_string(),
            })
            .unwrap(),
        ),
        None,
    );
    assert_eq!(result.unwrap(), "20000000000000000000".to_string());
}

#[test]
fn content_hash_uri_resolver() {
    let wrap_uri = format!("ens/goerli/test-wraps.eth");
    let mut config = ClientConfig::new();
    config
        .add(SystemClientConfig::default().into())
        .add(Web3ClientConfig::default().into());

    let client = Client::new(config.into());
    let result = client.invoke::<String>(
        &wrap_uri.parse().unwrap(),
        "toWei",
        Some(
            &to_vec(&ToWeiArgs {
                eth: "20".to_string(),
            })
            .unwrap(),
        ),
        None,
    );
    assert_eq!(result.unwrap(), "20000000000000000000".to_string());
}
