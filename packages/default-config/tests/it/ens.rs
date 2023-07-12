use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
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
    let mut config = PolywrapClientConfig::new();
    config
        .add(SystemClientConfig::default().into())
        .add(Web3ClientConfig::default().into());

    let client = PolywrapClient::new(config.into());
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
        None,
    );
    assert_eq!(result.unwrap(), "20000000000000000000".to_string());
}

#[test]
fn content_hash_uri_resolver() {
    let wrap_uri = format!("ens/goerli/test-wraps.eth");
    let mut config = PolywrapClientConfig::new();
    config
        .add(SystemClientConfig::default().into())
        .add(Web3ClientConfig::default().into());

    let client = PolywrapClient::new(config.into());
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
        None,
    );
    assert_eq!(result.unwrap(), "20000000000000000000".to_string());
}
