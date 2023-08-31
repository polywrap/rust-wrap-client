use polywrap_client::{
    client::Client,
    resolvers::static_resolver::{StaticResolver, StaticResolverLike},
};
use polywrap_http_plugin::HttpPlugin;
use polywrap_plugin::*;
use std::sync::Arc;

mod get;
mod post;

pub fn get_client() -> Client {
    let http_plugin = HttpPlugin {};
    let package = Arc::new(PluginPackage::from(http_plugin));

    let resolver = StaticResolver::from(vec![StaticResolverLike::Package(
        uri!("plugin/http"),
        package,
    )]);

    Client::new(ClientConfig {
        resolver: Arc::new(resolver),
        interfaces: None,
        envs: None,
    })
}
