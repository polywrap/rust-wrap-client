use polywrap_client_builder::PolywrapClientConfig;
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_http_plugin::HttpPlugin;

use polywrap_core::{client::ClientConfig, macros::uri, uri::Uri};
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::package::PluginPackage;
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

use crate::embeds::{fs_resolver, http_resolver, ipfs_http_client, ipfs_resolver};

pub struct SystemClientConfig(PolywrapClientConfig);

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
        Self({
            PolywrapClientConfig {
                redirects: Some(HashMap::from([
                    (
                        uri!("wrap://ens/wraps.eth:http@1.1.0"),
                        uri!("plugin/http@1.1.0"),
                    ),
                    (
                        uri!("wrap://ens/wraps.eth:file-system@1.0.0"),
                        uri!("plugin/file-system@1.0.0"),
                    ),
                    (
                        uri!("wrapscan.io/polywrap/wrapscan-uri-resolver@1.0"),
                        uri!("http/wraps.wrapscan.io/r/polywrap/wrapscan-uri-resolver@1.0"),
                    ),
                ])),
                interfaces: Some(HashMap::from([
                    (
                        "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                        vec![
                            uri!("ens/wraps.eth:http-uri-resolver-ext@1.0.1"),
                            uri!("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1"),
                            uri!("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1"),
                            uri!("wrapscan.io/polywrap/wrapscan-uri-resolver@1.0")
                        ],
                    ),
                    (
                        "wrap://ens/wraps.eth:ipfs-http-client@1.0.0".to_string(),
                        vec![uri!("wrap://ens/wraps.eth:ipfs-http-client@1.0.0")],
                    ),
                ])),
                wrappers: Some(vec![
                    (
                        uri!("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1"),
                        Arc::new(fs_resolver::wasm_wrapper()),
                    ),
                    (
                        uri!("ens/wraps.eth:http-uri-resolver-ext@1.0.1"),
                        Arc::new(http_resolver::wasm_wrapper()),
                    ),
                    (
                        uri!("wrap://ens/wraps.eth:ipfs-http-client@1.0.0"),
                        Arc::new(ipfs_http_client::wasm_wrapper()),
                    ),
                    (
                        uri!("wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1"),
                        Arc::new(ipfs_resolver::wasm_wrapper()),
                    ),
                ]),
                packages: Some(vec![
                    (
                        uri!("plugin/file-system@1.0.0"),
                        Arc::new(PluginPackage::from(FileSystemPlugin {})),
                    ),
                    (
                        uri!("plugin/http@1.1.0"),
                        Arc::new(PluginPackage::from(HttpPlugin {})),
                    ),
                ]),
                envs: Some(HashMap::from([(
                    uri!("wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1"),
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
                ..Default::default()
            }
        })
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
