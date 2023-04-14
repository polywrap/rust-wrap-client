use std::sync::{Arc, Mutex};
use http_plugin::HttpPlugin;
use httpmock::{prelude::*, Method};
use polywrap_client::client::PolywrapClient;
use polywrap_core::resolvers::static_resolver::{StaticResolver, StaticResolverLike};
use polywrap_core::{
    client::ClientConfig, invoke::Invoker, resolvers::uri_resolution_context::UriPackage, uri::Uri,
};
use polywrap_msgpack::{msgpack};
use polywrap_plugin::package::PluginPackage;
use serde_json::{json, Value};

fn get_client() -> PolywrapClient {
    let http_plugin = HttpPlugin { };
    let plugin_pkg: PluginPackage = http_plugin.into();
    let package = Arc::new(Mutex::new(plugin_pkg));

    let resolver = StaticResolver::from(vec![StaticResolverLike::Package(UriPackage {
        uri: Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
        package,
    })]);

    PolywrapClient::new(ClientConfig {
        resolver: Arc::new(resolver),
        interfaces: None,
        envs: None,
    })
}

#[test]
fn get_method() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/api")
            .header("access-control-allow-origin", "*")
            .header("access-control-allow-credentials", "true");

        then.status(200).json_body(json!({"data": "test-response"}));
    });

    let response = get_client()
        .invoke_raw(
            &Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
            "get",
            Some(&msgpack!({
                "url": "https://yesno.wtf/api",
            })),
            None,
            None,
        )
        .unwrap();

    println!("{:#?}", response)
}

#[test]
fn post_method() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(Method::POST)
            .path("/api")
            .header("access-control-allow-origin", "*")
            .header("access-control-allow-credentials", "true");

        then.status(200).json_body(json!({"data": "test-response"}));
    });

    let response = get_client()
        .invoke_raw(
            &Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
            "post",
            Some(&msgpack!({
                "url": "https://reqbin.com/echo/post/json",
                "request": {
                    "responseType": "TEXT",
                    "body": "{\"id\": \"some\", \"value\": 123}"
                }
            })),
            None,
            None,
        )
        .unwrap();

    println!("{:#?}", response)
}
