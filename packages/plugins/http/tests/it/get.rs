use polywrap_core::uri::Uri;
use polywrap_http_plugin::wrap::types::Response;
use polywrap_msgpack::{msgpack, serialize};
use polywrap_plugin::{Map, JSON};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::get_client;

#[derive(Debug, Serialize, Deserialize)]
struct ExpectedResponse {
    id: u32,
}

#[test]
fn simple_get() {
    let response = get_client()
        .invoke::<Response>(
            &Uri::try_from("plugin/http").unwrap(),
            "get",
            Some(&msgpack!({
                "url": "https://jsonplaceholder.typicode.com/todos/1",
            })),
            None,
            None,
        )
        .unwrap();
    assert_eq!(response.status, 200);
    assert_ne!(response.body, None);
    let body: ExpectedResponse = JSON::from_str(&response.body.unwrap()).unwrap();
    assert_eq!(body.id, 1);
}

#[derive(Debug, Serialize, Deserialize)]
struct GetArgs {
    url: String,
    request: Request,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "urlParams")]
    url_params: Map<String, String>,
    #[serde(rename = "responseType")]
    response_type: String,
}

#[test]
fn params_get() {
    let mut params = Map(BTreeMap::new());
    params.0.insert("id".to_string(), "1".to_string());

    let args = GetArgs {
        url: "https://jsonplaceholder.typicode.com/todos".to_string(),
        request: Request {
            url_params: params,
            response_type: "TEXT".to_string(),
        },
    };
    let response = get_client()
        .invoke::<Response>(
            &Uri::try_from("plugin/http").unwrap(),
            "get",
            Some(&serialize(&args).unwrap()),
            None,
            None,
        )
        .unwrap();
    assert_eq!(response.status, 200);
    assert_ne!(response.body, None);
    let response: JSON::Value = JSON::from_str(response.body.unwrap().as_str()).unwrap();
    if let JSON::Value::Array(r) = response {
        assert_eq!(r.len(), 1)
    } else {
        panic!("Error in params_get test")
    }
}
