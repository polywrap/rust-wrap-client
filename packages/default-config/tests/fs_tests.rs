use polywrap_client::{client::PolywrapClient};
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::uri::Uri;
use polywrap_msgpack::msgpack;

const SUBINVOKE_WRAP_URI: &str = "wrap://fs/./tests/wraps/subinvoke";

#[test]
fn sanity() {
    let mut config = PolywrapClientConfig::new();
    config
        .add(SystemClientConfig::default().into());

    let client = PolywrapClient::new(config.into());

    let result = client.invoke::<u32>(
        &Uri::try_from(SUBINVOKE_WRAP_URI).unwrap(),
        "add", 
        Some(&msgpack!({
            "a": 2,
            "b": 40
        })),
        None,
        None
    ).unwrap();

    assert_eq!(result, 42);
}
