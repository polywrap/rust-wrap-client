use polywrap_client::core::uri::Uri;
use polywrap_msgpack::serialize;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::wrap_types::get_client;

#[derive(Serialize, Deserialize)]
struct BytesMethodArgs {
    arg: Args,
}

#[derive(Serialize, Deserialize)]
struct Args {
    prop: ByteBuf,
}

#[test]
fn bytes_method() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bytes-type/implementations/rs", path)).unwrap();

    let client = get_client(None);
    let args = BytesMethodArgs {
        arg: Args {
            prop: ByteBuf::from("Argument Value".as_bytes()),
        },
    };
    let response = client
        .invoke::<ByteBuf>(
            &uri,
            "bytesMethod",
            Some(&serialize(&args).unwrap()),
            None,
            None,
        )
        .unwrap();
    let expected = "Argument Value Sanity!".as_bytes().to_vec();
    assert_eq!(response, expected);
}
