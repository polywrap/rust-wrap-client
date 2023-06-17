use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_http_plugin::HttpPlugin;

use polywrap_client_builder::types::BuilderConfig;
use polywrap_core::{client::UriRedirect, package::WrapPackage, uri::Uri, wrapper::Wrapper};
use polywrap_msgpack::msgpack;
use polywrap_plugin::package::PluginPackage;
use std::{collections::HashMap, sync::Arc};

pub mod embeds;

pub fn get_default_wrappers() -> Vec<(Uri, Arc<dyn Wrapper>)> {
    let ipfs_http_client_package = Arc::new(embeds::ipfs_http_client::wasm_wrapper());
    let ipfs_resolver_package = Arc::new(embeds::ipfs_resolver::wasm_wrapper());
    let fs_resolver_package = Arc::new(embeds::fs_resolver::wasm_wrapper());
    let http_resolver_package = Arc::new(embeds::http_resolver::wasm_wrapper());

    vec![
        (
            Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap(),
            ipfs_http_client_package,
        ),
        (
            Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(),
            ipfs_resolver_package,
        ),
        (
            Uri::try_from("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1").unwrap(),
            fs_resolver_package,
        ),
        (
            Uri::try_from("ens/wraps.eth:http-uri-resolver-ext@1.0.1").unwrap(),
            http_resolver_package,
        ),
    ]
}

pub fn get_default_plugins() -> Vec<(Uri, Arc<dyn WrapPackage>)> {
    let fs = FileSystemPlugin {};
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(fs_plugin_package);

    let http = HttpPlugin {};
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(http_plugin_package);

    vec![
        (
            Uri::try_from("plugin/file-system@1.0.0").unwrap(),
            fs_package,
        ),
        (Uri::try_from("plugin/http@1.1.0").unwrap(), http_package),
    ]
}

pub fn build() -> BuilderConfig {
    let mut interfaces = HashMap::new();
    interfaces.insert(
        "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
        vec![
            Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(),
            Uri::try_from("ens/wraps.eth:http-uri-resolver-ext@1.0.1").unwrap(),
            Uri::try_from("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1").unwrap(),
        ],
    );
    interfaces.insert(
        "wrap://ens/wraps.eth:ipfs-http-client@1.0.0".to_string(),
        vec![Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap()],
    );

    let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
    envs.insert(
        "wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1".to_string(),
        msgpack!({
            "provider": "https://ipfs.wrappers.io",
            "fallbackProviders": ["https://ipfs.io"],
            "retries": { "tryResolveUri": 2, "getFile": 2 },
        }),
    );

    let redirects: Vec<UriRedirect> = vec![UriRedirect {
        from: Uri::try_from("ens/wraps.eth:http@1.1.0").unwrap(),
        to: Uri::try_from("plugin/http@1.1.0").unwrap(),
    }];

    BuilderConfig {
        interfaces: Some(interfaces),
        envs: Some(envs),
        wrappers: Some(get_default_wrappers()),
        packages: Some(get_default_plugins()),
        redirects: Some(redirects),
        resolvers: None,
    }
}
