use polywrap_client::client::PolywrapClient;
use polywrap_core::{client::ClientConfig, macros::uri, uri::Uri};
use polywrap_http_plugin::HttpPlugin;
use polywrap_plugin::package::PluginPackage;
use polywrap_resolvers::static_resolver::{StaticResolver, StaticResolverLike};
use std::sync::Arc;

mod get;
mod post;

pub fn get_client() -> PolywrapClient {
    let http_plugin = HttpPlugin {};
    let package = Arc::new(PluginPackage::from(http_plugin));

    let resolver = StaticResolver::from(vec![StaticResolverLike::Package(
        uri!("plugin/http"),
        package,
    )]);

    PolywrapClient::new(ClientConfig {
        resolver: Arc::new(resolver),
        interfaces: None,
        envs: None,
    })
}
