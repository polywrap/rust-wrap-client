use num_bigint::BigInt;
use polywrap_client::client::Client;
use polywrap_client::core::uri::Uri;
use polywrap_msgpack_serde::to_vec;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Serialize;

use crate::wrap_types::get_client;

fn get_client_and_uri() -> (Client, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = format!("fs/{}/bigint-type/implementations/rs", path)
        .parse()
        .unwrap();

    (get_client(None), uri)
}

#[derive(Serialize)]
struct MethodArgs {
    arg1: String,
    arg2: Option<String>,
    obj: ArgsObject,
}

#[derive(Serialize)]
struct ArgsObject {
    prop1: String,
    prop2: Option<String>,
}

#[test]
fn method_without_optional_arguments() {
    let (client, uri) = get_client_and_uri();
    let response = client
        .invoke::<String>(
            &uri,
            "method",
            Some(
                &to_vec(&MethodArgs {
                    arg1: "123456789123456789".to_string(),
                    arg2: None,
                    obj: ArgsObject {
                        prop1: "987654321987654321".to_string(),
                        prop2: None,
                    },
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();
    let expected = "123456789123456789".parse::<BigInt>().unwrap()
        * "987654321987654321".parse::<BigInt>().unwrap();
    assert_eq!(response, expected.to_string());
}

#[test]
fn method_with_optional_arguments() {
    let (client, uri) = get_client_and_uri();
    let response = client
        .invoke::<String>(
            &uri,
            "method",
            Some(
                &to_vec(&MethodArgs {
                    arg1: "123456789123456789".to_string(),
                    arg2: Some("123456789123456789123456789123456789".to_string()),
                    obj: ArgsObject {
                        prop1: "987654321987654321".to_string(),
                        prop2: Some("987654321987654321987654321987654321".to_string()),
                    },
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    let expected = "123456789123456789".parse::<BigInt>().unwrap()
        * "123456789123456789123456789123456789"
            .parse::<BigInt>()
            .unwrap()
        * "987654321987654321".parse::<BigInt>().unwrap()
        * "987654321987654321987654321987654321"
            .parse::<BigInt>()
            .unwrap();
    assert_eq!(response, expected.to_string());
}
