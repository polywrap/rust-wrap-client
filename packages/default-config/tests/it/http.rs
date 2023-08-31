use core::panic;

use polywrap_client::client::PolywrapClient;
use polywrap_client_default_config::SystemClientConfig;
use polywrap_msgpack_serde::to_vec;
use serde::Serialize;

const URI: &str =
    "http/https://raw.githubusercontent.com/polywrap/client-readiness/main/wraps/public";

#[derive(Serialize)]
struct Args {
    first: u32,
    second: u32,
}

#[test]
fn sanity() {
    let client = PolywrapClient::new(SystemClientConfig::default().into());

    let result = client
        .invoke::<u32>(
            &URI.parse().unwrap(),
            "i8Method",
            Some(
                &to_vec(&Args {
                    first: 2,
                    second: 40,
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, 42);

    let client = PolywrapClient::new(SystemClientConfig::precompiled().into());

    let result = client
        .invoke::<u32>(
            &URI.parse().unwrap(),
            "i8Method",
            Some(
                &to_vec(&Args {
                    first: 2,
                    second: 40,
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, 42);
}
