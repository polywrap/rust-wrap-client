extern crate polywrap_client;
extern crate polywrap_client_builder;
extern crate polywrap_client_default_config;
extern crate polywrap_core;
extern crate polywrap_http_plugin;
extern crate polywrap_msgpack_serde;
extern crate polywrap_plugin;
extern crate serde;

use std::{collections::HashMap, sync::Arc};

use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_core::{client::ClientConfigBuilder, error::Error, macros::uri, uri::Uri};
use polywrap_http_plugin::HttpPlugin;
use polywrap_msgpack_serde::{
    to_vec,
    JSON::{self, json},
};
use polywrap_plugin::package::PluginPackage;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GetArgs {
    url: String,
}

#[derive(Serialize)]
struct Request {
    body: String,
    #[serde(rename = "responseType")]
    response_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    status: i64,
    #[serde(rename = "statusText")]
    status_text: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

#[derive(Serialize)]
struct PostArgs {
    url: String,
    request: Request,
}

fn main() {
    let uri = uri!("wrapscan.io/polywrap/http@1.0");
    let mut config = PolywrapClientConfig::new();
    let http_package = PluginPackage::from(HttpPlugin {});

    config.add_package(uri.clone(), Arc::new(http_package));

    let client = PolywrapClient::new(config.build());
    let get_result: Result<Response, Error> = client.invoke(
        &uri,
        "get",
        Some(
            &to_vec(&GetArgs {
                url: "https://httpbin.org/get".to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if get_result.is_err() {
        panic!(
            "Error with get request: {}",
            &get_result.unwrap_err().to_string()
        )
    }

    println!("Get method response: {:#?}", get_result.unwrap());

    let post_result: Result<Response, Error> = client.invoke(
        &uri,
        "post",
        Some(
            &to_vec(&PostArgs {
                url: "https://httpbin.org/post".to_string(),
                request: Request {
                    body: JSON::to_string(&json!({ "item": "Gello workd!" })).unwrap(),
                    response_type: "TEXT".to_string(),
                },
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if post_result.is_err() {
        panic!(
            "Error with post request: {}",
            &post_result.unwrap_err().to_string()
        )
    }
    println!("Post method response: {:#?}", post_result.unwrap());
}
