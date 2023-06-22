use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_msgpack::encode;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Serialize;
use serde_json::json;

use super::get_client;

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/object-type/implementations/rs", path)).unwrap();
    (get_client(None), uri)
}

#[derive(Serialize)]
struct Nested {
    prop: String,
}

#[derive(Serialize)]
struct Arg1 {
    prop: Option<String>,
    nested: Nested,
}

#[derive(Serialize)]
struct Arg2 {
    prop: String,
    circular: Nested,
}

#[derive(Serialize)]
struct Arg3 {
    #[serde(with = "serde_bytes")]
    prop: Vec<u8>,
}

#[derive(Serialize)]
struct MethodOneArgs {
    arg1: Arg1,
    arg2: Option<Arg2>,
}

#[test]
fn without_optional_argument_and_return_array_of_object() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<Vec<serde_json::Value>>(
            &uri,
            "method1",
            Some(
                &encode(&MethodOneArgs {
                    arg1: Arg1 {
                        prop: Some("arg1 prop".to_string()),
                        nested: Nested {
                            prop: "arg1 nested prop".to_string(),
                        },
                    },
                    arg2: None,
                })
                .unwrap(),
            ),
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
            Some(
                &encode(&MethodOneArgs {
                    arg1: Arg1 {
                        prop: Some("arg1 prop".to_string()),
                        nested: Nested {
                            prop: "arg1 nested prop".to_string(),
                        },
                    },
                    arg2: Some(Arg2 {
                        prop: "arg2 prop".to_string(),
                        circular: Nested {
                            prop: "arg2 circular prop".to_string(),
                        },
                    }),
                })
                .unwrap(),
            ),
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

#[derive(Serialize)]
struct MethodTwoArgs {
    arg: Arg1,
}

#[test]
fn returns_optional_return_value() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<Option<serde_json::Value>>(
            &uri,
            "method2",
            Some(
                &encode(&MethodTwoArgs {
                    arg: Arg1 {
                        prop: Some("arg1 prop".to_string()),
                        nested: Nested {
                            prop: "arg1 nested prop".to_string(),
                        },
                    },
                })
                .unwrap(),
            ),
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
            Some(
                &encode(&MethodTwoArgs {
                    arg: Arg1 {
                        prop: None,
                        nested: Nested {
                            prop: "arg1 nested prop".to_string(),
                        },
                    },
                })
                .unwrap(),
            ),
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
            Some(
                &encode(&MethodTwoArgs {
                    arg: Arg1 {
                        prop: Some("arg prop".to_string()),
                        nested: Nested {
                            prop: "arg nested prop".to_string(),
                        },
                    },
                })
                .unwrap(),
            ),
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

#[derive(Serialize)]
struct MethodThreeArgs {
    arg: Arg3,
}

#[test]
fn not_optional_args_and_returns_not_optional_object() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<serde_json::Value>(
            &uri,
            "method4",
            Some(
                &encode(&MethodThreeArgs {
                    arg: Arg3 {
                        prop: [49, 50, 51, 52].to_vec(),
                    },
                })
                .unwrap(),
            ),
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
