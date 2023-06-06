use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;
use serde_json::json;

use super::get_client;

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/object-type/implementations/rs", path)).unwrap();
    (get_client(None), uri)
}

#[test]
fn without_optional_argument_and_return_array_of_object() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<Vec<serde_json::Value>>(
            &uri,
            "method1",
            Some(&msgpack!({
                "arg1": {
                    "prop": "arg1 prop",
                    "nested": {
                        "prop": "arg1 nested prop",
                    },
                },
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        response,
        vec![
            json!({
                "prop": "arg1 prop",
                "nested": {
                    "prop": "arg1 nested prop",
                },
            }),
            json!({
                "prop": "",
                "nested": {
                    "prop": "",
                },
            }),
        ]
    );
}

#[test]
fn with_optional_argument_and_return_array_of_object() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<Vec<serde_json::Value>>(
            &uri,
            "method1",
            Some(&msgpack!({
                "arg1": {
                    "prop": "arg1 prop",
                    "nested": {
                        "prop": "arg1 nested prop",
                    },
                },
                "arg2": {
                    "prop": "arg2 prop",
                    "circular": {
                        "prop": "arg2 circular prop",
                    },
                },
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        response,
        vec![
            json!({
                "prop": "arg1 prop",
                "nested": {
                    "prop": "arg1 nested prop",
                },
            }),
            json!({
                "prop": "arg2 prop",
                "nested": {
                    "prop": "arg2 circular prop",
                },
            }),
        ]
    );
}

#[test]
fn returns_optional_return_value() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<Option<serde_json::Value>>(
            &uri,
            "method2",
            Some(&msgpack!({
                "arg": {
                    "prop": "arg prop",
                    "nested": {
                        "prop": "arg nested prop",
                    },
                },
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        response.unwrap(),
        json!({
            "prop": "arg prop",
            "nested": {
                "prop": "arg nested prop",
            },
        })
    );
}

#[test]
fn do_not_returns_optional_return_value() {
    let (client, uri) = get_client_and_uri();
    let response = client
        .invoke::<Option<serde_json::Value>>(
            &uri,
            "method2",
            Some(&msgpack!({
                "arg": {
                    "prop": "null",
                    "nested": {
                        "prop": "arg nested prop",
                    },
                },
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(response, None);
}

#[test]
fn not_optional_args_and_returns_not_optional_array_of_objects() {
    let (client, uri) = get_client_and_uri();
    let method3 = client
        .invoke::<Vec<serde_json::Value>>(
            &uri,
            "method3",
            Some(&msgpack!({
                "arg": {
                    "prop": "arg prop",
                    "nested": {
                        "prop": "arg nested prop",
                    },
                },
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        method3,
        vec![
            serde_json::Value::Null,
            json!({
                "prop": "arg prop",
                "nested": {
                    "prop": "arg nested prop",
                },
            }),
        ]
    );
}

#[test]
fn not_optional_args_and_returns_not_optional_object() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<serde_json::Value>(
            &uri,
            "method4",
            Some(&msgpack!({
                "arg": {
                    "prop": [49, 50, 51, 52],
                },
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        response,
        json!({
            "prop": "1234",
            "nested": {
                "prop": "nested prop",
            },
        })
    );
}
