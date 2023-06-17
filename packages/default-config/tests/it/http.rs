use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::uri::Uri;
use polywrap_msgpack::msgpack;

#[test]
fn sanity() {
    let uri = format!(
        "http/https://raw.githubusercontent.com/polywrap/client-readiness/main/wraps/public"
    );
    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());

    let client = PolywrapClient::new(config.into());

    let result = client
        .invoke::<u32>(
            &Uri::try_from(uri).unwrap(),
            "i8Method",
            Some(&msgpack!({
                "first": 2,
                "second": 40
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, 42);
}
