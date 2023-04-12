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
fn object_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/object-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let method1a = client.invoke::<Vec<serde_json::Value>>(
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
        None
    ).unwrap();

    assert_eq!(method1a, vec![
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
    ]);

    let method1b = client.invoke::<Vec<serde_json::Value>>(
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
        None
    ).unwrap();

    assert_eq!(method1b, vec![
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
    ]);

    let method2a = client.invoke::<Option<serde_json::Value>>(
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
        None
    ).unwrap();

    assert_eq!(method2a.unwrap(), json!({
        "prop": "arg prop",
        "nested": {
            "prop": "arg nested prop",
        },
    }));

    let method2b = client.invoke::<Option<serde_json::Value>>(
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
        None
    ).unwrap();

    assert_eq!(method2b, None);

    let method3 = client.invoke::<Vec<serde_json::Value>>(
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
        None
    ).unwrap();

    assert_eq!(method3, vec![
        serde_json::Value::Null,
        json!({
            "prop": "arg prop",
            "nested": {
                "prop": "arg nested prop",
            },
        }),
    ]);

    let method5 = client.invoke::<serde_json::Value>(
        &uri,
        "method5",
        Some(&msgpack!({
            "arg": {
                "prop": [49, 50, 51, 52],
            },
        })),
        None,
        None
    ).unwrap();

    assert_eq!(method5, json!({
        "prop": "1234",
        "nested": {
            "prop": "nested prop",
        },
    }));
}
