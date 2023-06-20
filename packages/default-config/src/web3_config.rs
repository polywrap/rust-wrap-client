use polywrap_client_builder::PolywrapClientConfig;
use polywrap_core::{client::ClientConfig, uri::Uri, wrapper::Wrapper};
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
                let mut interfaces = HashMap::new();
                interfaces.insert(
                    "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                    vec![Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap()],
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

                PolywrapClientConfig {
                    interfaces: Some(interfaces),
                    envs: Some(envs),
                    wrappers: Some(get_default_wrappers()),
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

pub fn get_default_wrappers() -> Vec<(Uri, Arc<dyn Wrapper>)> {
    let ipfs_http_client_package = Arc::new(ipfs_http_client::wasm_wrapper());
    let ipfs_resolver_package = Arc::new(ipfs_resolver::wasm_wrapper());

    vec![
        (
            Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap(),
            ipfs_http_client_package,
        ),
        (
            Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(),
            ipfs_resolver_package,
        ),
    ]
}
