use std::sync::{Mutex, Arc};

use polywrap_client::{client::PolywrapClient, builder::types::ClientConfigHandler};
use polywrap_core::{uri::Uri, resolution::uri_resolution_context::UriResolutionContext};
use polywrap_msgpack::msgpack;
mod utils;

#[test]
fn client_sanity() {
    let config = polywrap_client_default_config::build();
    let client = PolywrapClient::new(config.build());

    let context = UriResolutionContext::new();
    let context = Arc::new(Mutex::new(context));
    let result = client.invoke::<u32>(
        &Uri::try_from("wrap://ipfs/Qmf7jukQhTQekdSgKfdnFtB6ERTN6V7aT4oYpzesDyr2cS").unwrap(),
        "add", 
        Some(&msgpack!({
            "a": 2,
            "b": 40
        })),
        None,
        Some(context.clone())
    ).unwrap();

    // println!("Result: {:?}", context.lock().unwrap().get_history());
    assert_eq!(result, 43);

    panic!("Test")
}