use polywrap_core::{macros::uri, uri::Uri};
use polywrap_http_plugin::wrap::types::Response;
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::{Map, JSON};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::get_client;

#[derive(Debug, Serialize, Deserialize)]
struct ExpectedResponse {
    id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArgsGet {
    url: String,
}

#[test]
fn simple_get() {
    let response = get_client()
        .invoke::<Response>(
            &uri!("plugin/http"),
            "get",
            Some(
                &to_vec(&ArgsGet {
                    url: "https://jsonplaceholder.typicode.com/todos/1".to_string(),
                })
                .unwrap(),
            ),
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
    let mut params = BTreeMap::new();
    params.insert("id".to_string(), "1".to_string());

    let args = GetArgs {
        url: "https://jsonplaceholder.typicode.com/todos".to_string(),
        request: Request {
            url_params: params,
            response_type: "TEXT".to_string(),
        },
    };
    let response = get_client()
        .invoke::<Response>(
            &uri!("plugin/http"),
            "get",
            Some(&to_vec(&args).unwrap()),
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
