use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_msgpack::encode;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

#[derive(Serialize)]
struct ParseArgs {
    value: String,
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
            Some(
                &encode(&ParseArgs {
                    value: value.clone(),
                })
                .unwrap(),
            ),
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
            Some(&encode(&StringifyArgs { values }).unwrap()),
            None,
            None,
        )
        .unwrap();

    assert_eq!(stringify_response, "{\"bar\":\"foo\"}{\"baz\":\"fuz\"}");
}

#[derive(Serialize)]
struct StringifyObjectArgs {
    object: Value,
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
            Some(
                &encode(&StringifyObjectArgs {
                    object: object.clone(),
                })
                .unwrap(),
            ),
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

#[derive(Serialize)]
struct MethodJSONArgs {
    valueA: i64,
    valueB: String,
    valueC: bool,
}

#[derive(Serialize)]
struct ParseReservedArgs {
    json: String,
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
            Some(
                &encode(&MethodJSONArgs {
                    valueA: json["valueA"].as_i64().unwrap(),
                    valueB: json["valueB"].as_str().unwrap().to_string(),
                    valueC: json["valueC"].as_bool().unwrap(),
                })
                .unwrap(),
            ),
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
            Some(
                &encode(&ParseReservedArgs {
                    json: reserved.to_string(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(parse_reserved_response.to_string(), reserved.to_string());
}

#[derive(Serialize)]
struct StringifyReservedArgs {
    reserved: Reserved,
}

#[derive(Serialize)]
struct Reserved {
    r#const: String,
    r#if: bool,
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
            Some(
                &encode(&StringifyReservedArgs {
                    reserved: Reserved {
                        r#const: "hello".to_string(),
                        r#if: true,
                    },
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(stringify_reserved_response, reserved.to_string());
}
