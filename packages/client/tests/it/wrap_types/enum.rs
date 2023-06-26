use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_msgpack::encode;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Serialize;

use super::get_client;

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/enum-type/implementations/rs", path)).unwrap();

    (get_client(None), uri)
}

#[derive(Serialize)]
struct MethodOneArgs {
    en: u32,
    optEnum: Option<u32>,
}

#[test]
fn method_one_success() {
    let (client, uri) = get_client_and_uri();
    let response = client
        .invoke::<i32>(
            &uri,
            "method1",
            Some(
                &encode(&MethodOneArgs {
                    en: 2,
                    optEnum: Some(1),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();
    assert_eq!(response, 2);
}

#[test]
fn method_one_panic_invalid_value() {
    let (client, uri) = get_client_and_uri();
    // TODO: Panics instead of returning Result
    let response = client.invoke::<i32>(
        &uri,
        "method1",
        Some(
            &encode(&MethodOneArgs {
                en: 2,
                optEnum: None,
            })
            .unwrap(),
        ),
        None,
        None,
    );
    assert!(response
        .unwrap_err()
        .to_string()
        .contains("__wrap_abort: Invalid value for enum 'SanityEnum': 5"));
}

// #[derive(Serialize)]
// struct MethodTwoArgs {
//     enumArray: ,
//     optEnum: Option<u32>,
// }


// #[test]
// fn method_two_success() {
//     let (client, uri) = get_client_and_uri();
//     let response = client
//         .invoke::<Vec<i32>>(
//             &uri,
//             "method2",
//             Some(&msgpack!({
//                 "enumArray": ["OPTION1", 0, "OPTION3"],
//             })),
//             None,
//             None,
//         )
//         .unwrap();
//     assert_eq!(response, vec![0, 0, 2]);
// }
