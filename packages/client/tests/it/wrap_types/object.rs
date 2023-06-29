use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_msgpack_serde::to_vec;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};

use super::get_client;

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/object-type/implementations/rs", path)).unwrap();
    (get_client(None), uri)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
        .invoke::<Vec<Output>>(
            &uri,
            "method1",
            Some(
                &to_vec(&MethodOneArgs {
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
            Output {
                prop: "arg1 prop".to_string(),
                nested: Nested {
                    prop: "arg1 nested prop".to_string(),
                },
            },
            Output {
                prop: "".to_string(),
                nested: Nested {
                    prop: "".to_string(),
                },
            },
        ]
    );
}

#[test]
fn with_optional_argument_and_return_array_of_object() {
    let (client, uri) = get_client_and_uri();

    let response = client
        .invoke::<Vec<Output>>(
            &uri,
            "method1",
            Some(
                &to_vec(&MethodOneArgs {
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
            Output {
                prop: "arg1 prop".to_string(),
                nested: Nested {
                    prop: "arg1 nested prop".to_string(),
                },
            },
            Output {
                prop: "arg2 prop".to_string(),
                nested: Nested {
                    prop: "arg2 circular prop".to_string(),
                },
            },
        ]
    );
}

#[derive(Serialize)]
struct MethodTwoArgs {
    arg: Arg1,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Output {
    prop: String,
    nested: Nested,
}

#[test]
fn returns_optional_return_value() {
    let (client, uri) = get_client_and_uri();

    let args_to_vecd = &to_vec(&MethodTwoArgs {
        arg: Arg1 {
            prop: Some("arg1 prop".to_string()),
            nested: Nested {
                prop: "arg1 nested prop".to_string(),
            },
        },
    })
    .unwrap();
    let response = client
        .invoke::<Option<Output>>(&uri, "method2", Some(args_to_vecd), None, None)
        .unwrap();

    assert_eq!(
        response.unwrap(),
        Output {
            prop: "arg1 prop".to_string(),
            nested: Nested {
                prop: "arg1 nested prop".to_string(),
            }
        }
    );
}

#[test]
fn do_not_returns_optional_return_value() {
    let (client, uri) = get_client_and_uri();
    let to_vecd_args = &to_vec(&MethodTwoArgs {
        arg: Arg1 {
            prop: Some("null".to_string()),
            nested: Nested {
                prop: "arg nested prop".to_string(),
            },
        },
    })
    .unwrap();
    let response = client
        .invoke::<Option<Output>>(&uri, "method2", Some(to_vecd_args), None, None)
        .unwrap();

    assert_eq!(response, None);
}

#[test]
fn not_optional_args_and_returns_not_optional_array_of_objects() {
    let (client, uri) = get_client_and_uri();
    let method3 = client
        .invoke::<Vec<Option<Output>>>(
            &uri,
            "method3",
            Some(
                &to_vec(&MethodTwoArgs {
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
            None,
            Some(Output {
                prop: "arg prop".to_string(),
                nested: Nested {
                    prop: "arg nested prop".to_string(),
                }
            })
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
        .invoke::<Output>(
            &uri,
            "method4",
            Some(
                &to_vec(&MethodThreeArgs {
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
        Output {
            prop: "1234".to_string(),
            nested: Nested {
                prop: "nested prop".to_string(),
            },
        }
    );
}
