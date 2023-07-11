use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};
use polywrap_msgpack_serde::to_vec;
use serde::Serialize;

#[derive(Serialize)]
pub struct Keccak256Args {
    pub message: String,
}

#[test]
fn sanity() {
    let wrap_uri = format!("ens/wraps.eth:sha3@1.0.0");
    let mut config = PolywrapClientConfig::new();
    config
        .add(SystemClientConfig::default().into())
        .add(Web3ClientConfig::default().into());

    let client = PolywrapClient::new(config.into());

    let result = client.invoke::<u32>(
        &wrap_uri.parse().unwrap(),
        "keccak_256",
        Some(
            &to_vec(&Keccak256Args {
                message: "test message to hash".to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );
    print!("{:?}", &result);
    assert!(result.is_ok());
}
