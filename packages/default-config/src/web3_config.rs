use polywrap_client_builder::PolywrapClientConfig;
use polywrap_core::{client::ClientConfig, macros::uri, uri::Uri};
use polywrap_msgpack::{encode};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

use crate::embeds::{ipfs_http_client, ipfs_resolver};

pub struct Web3ClientConfig(PolywrapClientConfig);

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
impl Default for Web3ClientConfig {
    fn default() -> Self {
        Self(PolywrapClientConfig {
            interfaces: Some(HashMap::from([
                (
                    "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                    vec![uri!("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1")],
                ),
                (
                    "wrap://ens/wraps.eth:ipfs-http-client@1.0.0".to_string(),
                    vec![uri!("wrap://ens/wraps.eth:ipfs-http-client@1.0.0")],
                ),
            ])),
            envs: Some(HashMap::from([(
                uri!("wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1"),
                encode(&IpfsEnv {
                    provider: "https://ipfs.wrappers.io".to_string(),
                    fallback_providers: vec!["https://ipfs.io".to_string()],
                    retries: Retries {
                        try_resolve_uri: 2,
                        get_file: 2,
                    },
                })
                .unwrap(),
            )])),
            wrappers: Some(vec![
                (
                    uri!("wrap://ens/wraps.eth:ipfs-http-client@1.0.0"),
                    Arc::new(ipfs_http_client::wasm_wrapper()),
                ),
                (
                    uri!("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1"),
                    Arc::new(ipfs_resolver::wasm_wrapper()),
                ),
            ]),
            ..Default::default()
        })
    }
}

impl Into<PolywrapClientConfig> for Web3ClientConfig {
    fn into(self) -> PolywrapClientConfig {
        self.0
    }
}

impl Into<ClientConfig> for Web3ClientConfig {
    fn into(self) -> ClientConfig {
        self.0.into()
    }
}
