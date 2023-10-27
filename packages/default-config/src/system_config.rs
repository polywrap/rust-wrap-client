use polywrap_client_builder::PolywrapClientConfig;
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_http_plugin::HttpPlugin;
use polywrap_logger_plugin::LoggerPlugin;

use polywrap_core::{client::ClientConfig, macros::uri, uri::Uri, package::WrapPackage};
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::package::PluginPackage;
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

use crate::embeds::{fs_resolver, http_resolver, ipfs_http_client, ipfs_resolver};

/// The default system config for the `Client`.
/// Includes plugins and support for Filesystem, HTTP and IPFS interaction
/// Also includes a Logger plugin
pub struct SystemClientConfig(PolywrapClientConfig);

impl SystemClientConfig {
    pub fn precompiled() -> Self {
        build_config(Some(vec![
            (
                uri!("wrapscan.io/polywrap/file-system-uri-resolver@1.0"),
                Arc::new(fs_resolver::precompiled_wasm_package()),
            ),
            (
                uri!("wrapscan.io/polywrap/http-uri-resolver@1.0"),
                Arc::new(http_resolver::precompiled_wasm_package()),
            ),
            (
                uri!("wrapscan.io/polywrap/ipfs-http-client@1.0"),
                Arc::new(ipfs_http_client::precompiled_wasm_package()),
            ),
            (
                uri!("wrapscan.io/polywrap/async-ipfs-uri-resolver@1.0"),
                Arc::new(ipfs_resolver::precompiled_wasm_package()),
            ),
        ]))
    }

    pub fn lazy() -> Self {
        build_config(Some(vec![
            (
                uri!("wrapscan.io/polywrap/file-system-uri-resolver@1.0"),
                Arc::new(fs_resolver::lazy_loaded_wasm_package()),
            ),
            (
                uri!("wrapscan.io/polywrap/http-uri-resolver@1.0"),
                Arc::new(http_resolver::lazy_loaded_wasm_package()),
            ),
            (
                uri!("wrapscan.io/polywrap/ipfs-http-client@1.0"),
                Arc::new(ipfs_http_client::lazy_loaded_wasm_package()),
            ),
            (
                uri!("wrapscan.io/polywrap/async-ipfs-uri-resolver@1.0"),
                Arc::new(ipfs_resolver::lazy_loaded_wasm_package()),
            ),
        ]))
    }
}

#[derive(Serialize)]
pub struct Retries {
    #[serde(rename = "tryResolveUri")]
    try_resolve_uri: u8,
    #[serde(rename = "getFile")]
    get_file: u8,
}

#[derive(Serialize)]
pub struct IpfsEnv {
    provider: String,
    #[serde(rename = "fallbackProviders")]
    fallback_providers: Vec<String>,
    retries: Retries,
}

impl Default for SystemClientConfig {
    fn default() -> Self {
        SystemClientConfig::lazy()
    }
}

impl Into<PolywrapClientConfig> for SystemClientConfig {
    fn into(self) -> PolywrapClientConfig {
        self.0
    }
}

impl Into<ClientConfig> for SystemClientConfig {
    fn into(self) -> ClientConfig {
        self.0.into()
    }
}

fn build_config(packages: Option<Vec<(Uri, Arc<dyn WrapPackage>)>>) -> SystemClientConfig {
    let plugins: Vec<(Uri, Arc<dyn WrapPackage>)> = vec![
        (
            uri!("plugin/file-system@1.0.0"),
            Arc::new(PluginPackage::from(FileSystemPlugin {})),
        ),
        (
            uri!("plugin/http@1.1.0"),
            Arc::new(PluginPackage::from(HttpPlugin {})),
        ),
        (
            uri!("wrapscan.io/polywrap/logger@1.0"),
            Arc::new(PluginPackage::from(LoggerPlugin::new(None))),
        ),
    ];

    let packages = match packages {
        Some(packages) => plugins.into_iter().chain(packages).collect(),
        None => plugins,
    };

    SystemClientConfig({
        PolywrapClientConfig {
            redirects: Some(HashMap::from([
                (
                    uri!("wrapscan.io/polywrap/http@1.0"),
                    uri!("plugin/http@1.1.0"),
                ),
                (
                    uri!("wrapscan.io/polywrap/file-system@1.0"),
                    uri!("plugin/file-system@1.0.0"),
                ),
                (
                    uri!("wrapscan.io/polywrap/wrapscan-uri-resolver@1.0"),
                    uri!("http/https://wraps.wrapscan.io/r/polywrap/wrapscan-uri-resolver@1.0"),
                ),
            ])),
            interfaces: Some(HashMap::from([
                (
                    uri!("wrapscan.io/polywrap/uri-resolver@1.0"),
                    vec![
                        uri!("wrapscan.io/polywrap/wrapscan-uri-resolver@1.0"),
                        uri!("wrapscan.io/polywrap/http-uri-resolver@1.0"),
                        uri!("wrapscan.io/polywrap/file-system-uri-resolver@1.0"),
                        uri!("wrapscan.io/polywrap/async-ipfs-uri-resolver@1.0"),
                    ],
                ),
                (
                    uri!("wrapscan.io/polywrap/ipfs-http-client@1.0"),
                    vec![uri!("wrapscan.io/polywrap/ipfs-http-client@1.0")],
                ),
                (
                    uri!("wrapscan.io/polywrap/logger@1.0"),
                    vec![uri!("wrapscan.io/polywrap/logger@1.0")],
                ),
            ])),
            envs: Some(HashMap::from([(
                uri!("wrapscan.io/polywrap/async-ipfs-uri-resolver@1.0"),
                to_vec(&IpfsEnv {
                    provider: "https://ipfs.wrappers.io".to_string(),
                    fallback_providers: vec!["https://ipfs.io".to_string()],
                    retries: Retries {
                        try_resolve_uri: 2,
                        get_file: 2,
                    },
                })
                .unwrap(),
            )])),
            packages: Some(packages),
            ..Default::default()
        }
    })
}