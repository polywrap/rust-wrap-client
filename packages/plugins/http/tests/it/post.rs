use polywrap_client::{
    core::uri::Uri,
    msgpack::msgpack,
    plugin::JSON::{from_str, json},
};
use polywrap_http_plugin::wrap::types::Response;
use serde::{Deserialize, Serialize};

use crate::get_client;

#[derive(Debug, Serialize, Deserialize)]
struct ExpectedResponse {
    id: u32,
    value: u32,
}

#[test]
fn post_method() {
    let body = json!({
        "value": 5
    });
    let response = get_client()
        .invoke::<Response>(
            &Uri::try_from("plugin/http").unwrap(),
            "post",
            Some(&msgpack!({
                "url": "https://jsonplaceholder.typicode.com/todos",
                "request": {
                    "responseType": "TEXT",
                    "body": body.to_string()
                }
            })),
            None,
            None,
        )
        .unwrap();

    assert_eq!(response.status, 201);
    assert_ne!(response.body, None);
    let expected_response: ExpectedResponse = from_str(&response.body.unwrap()).unwrap();
    assert_eq!(expected_response.id, 201);
    assert_eq!(expected_response.value, 5);
}
