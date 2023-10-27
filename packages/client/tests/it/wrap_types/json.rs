use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_msgpack_serde::{to_vec, wrappers::polywrap_json};
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
    let uri = format!("fs/{}/json-type/implementations/rs", path)
        .parse()
        .unwrap();

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
                &to_vec(&ParseArgs {
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
            Some(&to_vec(&StringifyArgs { values }).unwrap()),
            None,
            None,
        )
        .unwrap();

    assert_eq!(stringify_response, "{\"bar\":\"foo\"}{\"baz\":\"fuz\"}");
}

#[derive(Serialize)]
struct StringifyObjectArgs {
    object: Object,
}

#[allow(non_snake_case)]
#[derive(Serialize, Clone)]
struct Object {
    #[serde(with = "polywrap_json")]
    jsonA: Value,
    #[serde(with = "polywrap_json")]
    jsonB: Value,
}

#[test]
fn stringify_object() {
    let (client, uri) = get_client_and_uri();
    let object = Object {
        jsonA: json!({ "foo": "bar" }),
        jsonB: json!({ "fuz": "baz" }),
    };

    let stringify_object_response = client
        .invoke::<String>(
            &uri,
            "stringifyObject",
            Some(
                &to_vec(&StringifyObjectArgs {
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
        object.jsonA.to_string() + &object.jsonB.to_string()
    );
}

#[allow(non_snake_case)]
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
                &to_vec(&MethodJSONArgs {
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
        .invoke::<Reserved>(
            &uri,
            "parseReserved",
            Some(
                &to_vec(&ParseReservedArgs {
                    json: reserved.to_string(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();

    assert_eq!(
        parse_reserved_response,
        Reserved {
            r#const: "hello".to_string(),
            r#if: true
        }
    );
}

#[derive(Serialize)]
struct StringifyReservedArgs {
    reserved: Reserved,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
                &to_vec(&StringifyReservedArgs {
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
