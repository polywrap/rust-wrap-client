use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use polywrap_client::client::PolywrapClient;
use polywrap_client::builder::types::{BuilderConfig, ClientConfigHandler, ClientBuilder};
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;
use polywrap_core::resolvers::uri_resolution_context::UriPackage;
use polywrap_plugin::package::PluginPackage;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_tests_utils::memory_storage_plugin::MemoryStoragePlugin;
use num_bigint::BigInt;
use bigdecimal::BigDecimal as BigNumber;
use serde_json::json;

#[test]
fn json_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/json-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    // parse method
    let value = json!({
        "foo": "bar",
        "bar": "bar",
    }).to_string();

    let parse_response = client.invoke::<String>(
        &uri,
        "parse",
        Some(&msgpack!({
            "value": value.clone(),
        })),
        None,
        None
    ).unwrap();

    assert_eq!(parse_response, value);

    // TODO: how do I pass an [JSON!]!
    // stringify method
    let values = vec![
        json!({ "bar": "foo" }).to_string(),
        json!({ "baz": "fuz" }).to_string(),
    ];

    let stringify_response = client.invoke::<String>(
        &uri,
        "stringify",
        Some(&msgpack!({
            "values": values
        })),
        None,
        None
    ).unwrap();

    assert_eq!(stringify_response, values.join(""));

    // stringifyObject method
    let object = json!({
        "jsonA": json!({ "foo": "bar" }).to_string(),
        "jsonB": json!({ "fuz": "baz" }).to_string(),
    });

    // TODO: how can i pass object directly?
    let stringify_object_response = client.invoke::<String>(
        &uri,
        "stringifyObject",
        Some(&msgpack!({
            "object": {
                "jsonA": json!({ "foo": "bar" }).to_string(),
                "jsonB": json!({ "fuz": "baz" }).to_string(),
            }
        })),
        None,
        None
    ).unwrap();

    assert_eq!(
        stringify_object_response,
        object["jsonA"].as_str().unwrap().to_string() + &object["jsonB"].as_str().unwrap().to_string()
    );

    // methodJSON method
    let json = json!({
        "valueA": 5,
        "valueB": "foo",
        "valueC": true,
    });

    let method_json_response = client.invoke::<String>(
        &uri,
        "methodJSON",
        Some(&msgpack!({
            "valueA": json["valueA"].as_i64().unwrap(),
            "valueB": json["valueB"].as_str().unwrap(),
            "valueC": json["valueC"].as_bool().unwrap(),
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method_json_response, json.to_string());


    // parseReserved method
    let reserved = json!({
            "const": "hello",
            "if": true,
        });

    let parse_reserved_response = client.invoke::<serde_json::Value>(
        &uri,
        "parseReserved",
        Some(&msgpack!({
            "json": reserved.to_string(),
        })),
        None,
        None
    ).unwrap();

    assert_eq!(parse_reserved_response.to_string(), reserved.to_string());

    // TODO: how can i pass reserved directly?
    // stringifyReserved method
    let stringify_reserved_response = client.invoke::<String>(
        &uri,
        "stringifyReserved",
        Some(&msgpack!({
            "reserved": {
                "const": "hello",
                "if": true,
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(stringify_reserved_response, reserved.to_string());
}

