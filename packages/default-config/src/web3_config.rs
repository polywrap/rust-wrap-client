use polywrap_client_builder::PolywrapClientConfig;
use polywrap_core_macros::uri;
use polywrap_core::{client::ClientConfig, uri::Uri};
use polywrap_msgpack::msgpack;
use std::{collections::HashMap, sync::Arc};

use crate::embeds::{ipfs_http_client, ipfs_resolver};

pub struct Web3ClientConfig {
    inner_config: PolywrapClientConfig,
}

impl Default for Web3ClientConfig {
    fn default() -> Self {
        Self {
            inner_config: {
                PolywrapClientConfig {
                    interfaces: Some(HashMap::from([
                        (
                            "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                            vec![
                                uri!("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1"),
                            ],
                        ),
                        (
                            "wrap://ens/wraps.eth:ipfs-http-client@1.0.0".to_string(),
                            vec![uri!("wrap://ens/wraps.eth:ipfs-http-client@1.0.0")],
                        ),
                    ])),
                    envs: Some(HashMap::from([(
                        uri!("wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1"),
                        msgpack!({
                            "provider": "https://ipfs.wrappers.io",
                            "fallbackProviders": ["https://ipfs.io"],
                            "retries": { "tryResolveUri": 2, "getFile": 2 },
                        }),
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
                }
            },
        }
    }
}

impl Into<PolywrapClientConfig> for Web3ClientConfig {
    fn into(self) -> PolywrapClientConfig {
        self.inner_config
    }
}

impl Into<ClientConfig> for Web3ClientConfig {
    fn into(self) -> ClientConfig {
        self.inner_config.into()
    }
}
