use fs_plugin_rs::FileSystemPlugin;
use http_plugin_rs::HttpPlugin;

use polywrap_client_builder::types::BuilderConfig;
use polywrap_core::{
    client::UriRedirect, env::Envs, resolvers::uri_resolution_context::UriPackage,
    resolvers::uri_resolution_context::UriWrapper, uri::Uri,
};
use polywrap_plugin::package::PluginPackage;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub mod embeds;

pub fn get_default_wrappers() -> Vec<UriWrapper> {
    let ipfs_http_client_package = Arc::new(Mutex::new(embeds::ipfs_http_client::wasm_wrapper()));
    let ipfs_resolver_package = Arc::new(Mutex::new(embeds::ipfs_resolver::wasm_wrapper()));
    let fs_resolver_package = Arc::new(Mutex::new(embeds::fs_resolver::wasm_wrapper()));
    let http_resolver_package = Arc::new(Mutex::new(embeds::http_resolver::wasm_wrapper()));

    vec![
        UriWrapper {
            uri: Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap(),
            wrapper: ipfs_http_client_package,
        },
        UriWrapper {
            uri: Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(),
            wrapper: ipfs_resolver_package,
        },
        UriWrapper {
            uri: Uri::try_from("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1").unwrap(),
            wrapper: fs_resolver_package,
        },
        UriWrapper {
            uri: Uri::try_from("ens/wraps.eth:http-uri-resolver-ext@1.0.1").unwrap(),
            wrapper: http_resolver_package,
        },
    ]
}

pub fn get_default_plugins() -> Vec<UriPackage> {
    let fs = FileSystemPlugin {};
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let http = HttpPlugin {};
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(Mutex::new(http_plugin_package));

    vec![
        UriPackage {
            uri: Uri::try_from("plugin/file-system@1.0.0").unwrap(),
            package: fs_package,
        },
        UriPackage {
            uri: Uri::try_from("plugin/http@1.1.0").unwrap(),
            package: http_package,
        },
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

    let mut envs: Envs = HashMap::new();
    envs.insert(
        "ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1".to_string(),
        json!({
            "provider": "https://ipfs.wrappers.io",
            "fallbackProviders": ["https://ipfs.io"],
            "retries": { "tryResolveUri": 2, "getFile": 2 },
        }),
    );

    let redirects: Vec<UriRedirect> = vec![
        UriRedirect {
            from: Uri::try_from("ens/wraps.eth:http@1.1.0").unwrap(),
            to: Uri::try_from("plugin/http@1.1.0").unwrap(),
        },
        UriRedirect {
            from: Uri::try_from("ens/wraps.eth:http@1.1.0").unwrap(),
            to: Uri::try_from("plugin/http@1.1.0").unwrap(),
        },
    ];

    BuilderConfig {
        interfaces: Some(interfaces),
        envs: Some(envs),
        wrappers: Some(get_default_wrappers()),
        packages: Some(get_default_plugins()),
        redirects: Some(redirects),
        resolvers: None,
    }
}
