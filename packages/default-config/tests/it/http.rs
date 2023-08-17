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
use std::time::Instant;
#[test]
fn sanity() {
    // measure time
    let now = Instant::now();

    let client = PolywrapClient::new(SystemClientConfig::default().into());
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
    let client = PolywrapClient::new(SystemClientConfig::default().into());
    let elapsed2 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed2 - elapsed1);
    let client = PolywrapClient::new(SystemClientConfig::default().into());


    // panic!("TEST");

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
    let elapsed3 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed3 - elapsed2);
    // panic!("TEST");

}
