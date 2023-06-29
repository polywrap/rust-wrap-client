use polywrap_http_plugin::HttpPlugin;
use polywrap_client::client::PolywrapClient;
use polywrap_core::{
    client::ClientConfig,
    uri::Uri,
};
use polywrap_resolvers::static_resolver::{StaticResolver, StaticResolverLike};
use polywrap_plugin::{package::PluginPackage};
use std::{sync::Arc};


mod get;
mod post;

pub fn get_client() -> PolywrapClient {
    let http_plugin = HttpPlugin {};
    let package = Arc::new(PluginPackage::from(http_plugin));

    let resolver = StaticResolver::from(vec![StaticResolverLike::Package(
        Uri::try_from("plugin/http").unwrap(),
        package,
    )]);

    PolywrapClient::new(ClientConfig {
        resolver: Arc::new(resolver),
        interfaces: None,
        envs: None,
    })
}