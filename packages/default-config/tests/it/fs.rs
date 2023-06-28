use polywrap_client::client::PolywrapClient;
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::uri::Uri;
use polywrap_msgpack::encode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ArgsAdd {
    pub a: u32,
    pub b: u32,
}

#[test]
fn sanity() {
    let subinvoke_wrap_uri = format!("fs/./tests/it/wrapper");

    let client = PolywrapClient::new(SystemClientConfig::default().into());

    let result = client
        .invoke::<u32>(
            &Uri::try_from(subinvoke_wrap_uri).unwrap(),
            "add",
            Some(&encode(&ArgsAdd { a: 2, b: 40 }).unwrap()),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, 42);
}