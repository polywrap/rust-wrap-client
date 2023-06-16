use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_msgpack::serialize;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::get_client;

#[derive(Serialize, Deserialize)]
struct StringifyArgs {
    values: Vec<String>,
}

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/json-type/implementations/rs", path)).unwrap();

    (get_client(None), uri)
}
#[test]
fn parse() {
    let (client, uri) = get_client_and_uri();
    // parse method
    let value = json!({
        "foo": "bar",
        "bar": "bar",
    })
    .to_string();

    let parse_response = client
        .invoke::<String>(
            &uri,
            "parse",
            Some(&msgpack!({
                "value": value.clone(),
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(parse_response, value);
}

#[test]
fn stringify() {
    let (client, uri) = get_client_and_uri();
    let values = vec![
        json!({ "bar": "foo" }).to_string(),
        json!({ "baz": "fuz" }).to_string(),
    ];

    let stringify_response = client
        .invoke::<String>(
            &uri,
            "stringify",
            Some(&serialize(&StringifyArgs { values }).unwrap()),
            None,
            None,
        )
        .unwrap();

    assert_eq!(stringify_response, "{\"bar\":\"foo\"}{\"baz\":\"fuz\"}");
}

#[test]
fn stringify_object() {
    let (client, uri) = get_client_and_uri();
    let object = json!({
        "jsonA": json!({ "foo": "bar" }).to_string(),
        "jsonB": json!({ "fuz": "baz" }).to_string(),
    });

    let stringify_object_response = client
        .invoke::<String>(
            &uri,
            "stringifyObject",
            Some(&msgpack!({
                "object": {
                    "jsonA": json!({ "foo": "bar" }).to_string(),
                    "jsonB": json!({ "fuz": "baz" }).to_string(),
                }
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        stringify_object_response,
        object["jsonA"].as_str().unwrap().to_string()
            + &object["jsonB"].as_str().unwrap().to_string()
    );
}

#[test]
fn method_json() {
    let (client, uri) = get_client_and_uri();
    let json = json!({
        "valueA": 5,
        "valueB": "foo",
        "valueC": true,
    });

    let method_json_response = client
        .invoke::<String>(
            &uri,
            "methodJSON",
            Some(&msgpack!({
                "valueA": json["valueA"].as_i64().unwrap(),
                "valueB": json["valueB"].as_str().unwrap(),
                "valueC": json["valueC"].as_bool().unwrap(),
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(method_json_response, json.to_string());

    // parseReserved method
    let reserved = json!({
        "const": "hello",
        "if": true,
    });

    let parse_reserved_response = client
        .invoke::<serde_json::Value>(
            &uri,
            "parseReserved",
            Some(&msgpack!({
                "json": reserved.to_string(),
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(parse_reserved_response.to_string(), reserved.to_string());
}

#[test]
fn stringify_reserved() {
    let (client, uri) = get_client_and_uri();
    let reserved = json!({
        "const": "hello",
        "if": true,
    });
    let stringify_reserved_response = client
        .invoke::<String>(
            &uri,
            "stringifyReserved",
            Some(&msgpack!({
                "reserved": {
                    "const": "hello",
                    "if": true,
                },
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(stringify_reserved_response, reserved.to_string());
}
