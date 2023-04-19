use filesystem_plugin::FileSystemPlugin;
use fs_resolver_plugin::FileSystemResolverPlugin;
use http_plugin::HttpPlugin;
use http_resolver_plugin::HttpResolverPlugin;
use polywrap_client_builder::types::BuilderConfig;
use polywrap_core::env::Envs;
use polywrap_core::{client::UriRedirect, resolvers::uri_resolution_context::UriPackage, uri::Uri};
use polywrap_plugin::package::PluginPackage;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub mod embeds;

pub fn get_default_plugins() -> Vec<UriPackage> {
    let fs = FileSystemPlugin {};
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin {};
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let http = HttpPlugin {};
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(Mutex::new(http_plugin_package));

    let http_resolver = HttpResolverPlugin {};
    let http_resolver_plugin_package: PluginPackage = http_resolver.into();
    let http_resolver_package = Arc::new(Mutex::new(http_resolver_plugin_package));

    // let ipfs_http_client_package = Arc::new(Mutex::new(ipfsHttpClientPackage()));
    // let ipfs_resolver_package = Arc::new(Mutex::new(ipfsResolverPackage()));

    vec![
        // UriPackage {
        //     uri: Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap(),
        //     package: ipfs_http_client_package
        // },
        // UriPackage {
        //     uri: Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.0").unwrap(),
        //     package: ipfs_resolver_package
        // },
        UriPackage {
            uri: Uri::try_from("wrap://ens/fs.polywrap.eth").unwrap(),
            package: fs_package,
        },
        UriPackage {
            uri: Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
            package: fs_resolver_package,
        },
        UriPackage {
            uri: Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
            package: http_package,
        },
        UriPackage {
            uri: Uri::try_from("wrap://ens/http-resolver.polywrap.eth").unwrap(),
            package: http_resolver_package,
        },
    ]
}

pub fn build() -> BuilderConfig {
    let mut interfaces = HashMap::new();
    interfaces.insert(
        "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
        vec![
            // Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.0").unwrap(),
            Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
            Uri::try_from("wrap://ens/http-resolver.polywrap.eth").unwrap(),
        ],
    );
    // interfaces.insert(
    //     "wrap://ens/wraps.eth:ipfs-http-client@1.0.0".to_string(),
    //     vec![
    //         Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap(),
    //     ]
    // );

    let envs: Envs = HashMap::new();
    // envs.insert(
    //     "ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.0".to_string(),
    //     json!({
    //         "provider": "https://ipfs.wrappers.io",
    //         "fallbackProviders": ["https://ipfs.io"],
    //         "retries": { "tryResolveUri": 2, "getFile": 2 },
    //     })
    // );

    let mut redirects: Vec<UriRedirect> = Vec::new();
    redirects.push(UriRedirect {
        from: Uri::try_from("ens/wraps.eth:http@1.1.0").unwrap(),
        to: Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
    });

    BuilderConfig {
        interfaces: Some(interfaces),
        envs: Some(envs),
        wrappers: None,
        packages: Some(get_default_plugins()),
        redirects: Some(redirects),
        resolvers: None,
    }
}
