use bigdecimal::BigDecimal as BigNumber;
use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_msgpack_serde::to_vec;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Serialize;

use crate::wrap_types::get_client;

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

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = format!("fs/{}/bignumber-type/implementations/rs", path)
        .parse()
        .unwrap();

    (get_client(None), uri)
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
                    arg1: "1234.56789123456789".to_string(),
                    arg2: None,
                    obj: ArgsObject {
                        prop1: "98.7654321987654321".to_string(),
                        prop2: None,
                    },
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    let arg1 = "1234.56789123456789".parse::<BigNumber>().unwrap();
    let prop1 = "98.7654321987654321".parse::<BigNumber>().unwrap();
    let result = arg1 * prop1;
    assert_eq!(response, result.to_string());
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
                    arg1: "1234567.89123456789".to_string(),
                    arg2: Some("123456789123.456789123456789123456789".to_string()),
                    obj: ArgsObject {
                        prop1: "987654.321987654321".to_string(),
                        prop2: Some("987.654321987654321987654321987654321".to_string()),
                    },
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    let arg1 = "1234567.89123456789".parse::<BigNumber>().unwrap();
    let arg2 = "123456789123.456789123456789123456789"
        .parse::<BigNumber>()
        .unwrap();
    let prop1 = "987654.321987654321".parse::<BigNumber>().unwrap();
    let prop2 = "987.654321987654321987654321987654321"
        .parse::<BigNumber>()
        .unwrap();
    let result = arg1 * arg2 * prop1 * prop2;
    assert_eq!(response, result.to_string());
}
