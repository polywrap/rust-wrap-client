use polywrap_msgpack_serde::to_vec;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use polywrap_msgpack_serde::bytes;

use crate::wrap_types::get_client;

#[derive(Serialize, Deserialize)]
struct BytesMethodArgs {
    arg: Args,
}

#[derive(Serialize, Deserialize)]
struct Args {
    #[serde(with = "bytes")]
    prop: Vec<u8>,
}

#[test]
fn bytes_method() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = format!("fs/{}/bytes-type/implementations/rs", path)
        .parse()
        .unwrap();

    let client = get_client(None);
    let args = BytesMethodArgs {
        arg: Args {
            prop: "Argument Value".as_bytes().to_vec(),
        },
    };
    let response = client
        .invoke::<bytes::ByteBuf>(
            &uri,
            "bytesMethod",
            Some(&to_vec(&args).unwrap()),
            None,
            None,
        )
        .unwrap();
    let expected = "Argument Value Sanity!".as_bytes().to_vec();
    assert_eq!(response, expected);
}
