use polywrap_client::{builder::types::ClientConfigHandler, client::PolywrapClient};
use polywrap_core::uri::Uri;
use polywrap_msgpack::msgpack;

const SUBINVOKE_WRAP_URI: &str = "wrap://ipfs/Qmf7jukQhTQekdSgKfdnFtB6ERTN6V7aT4oYpzesDyr2cS";

#[test]
fn sanity() {
    let config = polywrap_client_default_config::build();
    let client = PolywrapClient::new(config.build());

    let result = client
        .invoke::<u32>(
            &Uri::try_from(SUBINVOKE_WRAP_URI).unwrap(),
            "add",
            Some(&msgpack!({
                "a": 2,
                "b": 40
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, 42);
}
