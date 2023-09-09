use polywrap_client::client::Client;
use polywrap_client_builder::{ClientConfig, ClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::{wrap_loader::WrapLoader, uri::Uri, macros::uri};

#[test]
fn sanity() {
    let mut config = ClientConfig::new();
    config
        .add(SystemClientConfig::precompiled().into());

    let client = Client::new(config.into());
    client
        .load_wrapper(
            &uri!("wrapscan.io/polywrap/wrapscan-uri-resolver@1.0.0"),
            None
        ).unwrap();
}
