use polywrap_client::{
    core::uri::Uri,
    plugin::JSON::{from_str, json},
};
use polywrap_http_plugin::wrap::types::{Request, Response};
use polywrap_msgpack::encode;
use serde::{Deserialize, Serialize};

use crate::get_client;

#[derive(Debug, Serialize, Deserialize)]
struct ExpectedResponse {
    id: u32,
    value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArgsPost {
    url: String,
    request: Request,
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
            Some(
                &encode(&ArgsPost {
                    url: "https://jsonplaceholder.typicode.com/todos".to_string(),
                    request: Request {
                        response_type: polywrap_http_plugin::wrap::types::ResponseType::TEXT,
                        body: Some(body.to_string()),
                        headers: None,
                        url_params: None,
                        form_data: None,
                        timeout: None,
                    },
                })
                .unwrap(),
            ),
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