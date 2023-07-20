use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::{wrap_loader::WrapLoader, uri::Uri, macros::uri};

#[test]
fn sanity() {
    let mut config = PolywrapClientConfig::new();
    config
        .add(SystemClientConfig::default().into());

    let client = PolywrapClient::new(config.into());
    client
        .load_wrapper(
            &uri!("wrapscan/polywrap/wrapscan-uri-resolver@1.0.0"),
            None
        ).unwrap();
}
