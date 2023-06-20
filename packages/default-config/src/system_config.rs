use polywrap_client_builder::PolywrapClientConfig;
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_http_plugin::HttpPlugin;

use polywrap_core::{client::ClientConfig, uri::Uri};
use polywrap_plugin::package::PluginPackage;
use std::{collections::HashMap, sync::Arc};

use crate::embeds::{fs_resolver, http_resolver};

pub struct SystemClientConfig {
    inner_config: PolywrapClientConfig,
}

impl Default for SystemClientConfig {
    fn default() -> Self {
        Self {
            inner_config: {
                PolywrapClientConfig {
                    redirects: Some(HashMap::from([
                        (
                            Uri::try_from("ens/wraps.eth:http@1.1.0").unwrap(),
                            Uri::try_from("plugin/http@1.1.0").unwrap(),
                        ),
                        (
                            Uri::try_from("wrap://ens/wraps.eth:file-system@1.0.0").unwrap(),
                            Uri::try_from("plugin/file-system@1.0.0").unwrap(),
                        ),
                    ])),
                    interfaces: Some(HashMap::from([(
                        "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                        vec![
                            Uri::try_from("ens/wraps.eth:http-uri-resolver-ext@1.0.1").unwrap(),
                            Uri::try_from("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1")
                                .unwrap(),
                        ],
                    )])),
                    wrappers: Some(vec![
                        (
                            Uri::try_from("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1")
                                .unwrap(),
                            Arc::new(fs_resolver::wasm_wrapper()),
                        ),
                        (
                            Uri::try_from("ens/wraps.eth:http-uri-resolver-ext@1.0.1").unwrap(),
                            Arc::new(http_resolver::wasm_wrapper()),
                        ),
                    ]),
                    packages: Some(vec![
                        (
                            Uri::try_from("plugin/file-system@1.0.0").unwrap(),
                            Arc::new(PluginPackage::from(FileSystemPlugin {})),
                        ),
                        (
                            Uri::try_from("plugin/http@1.1.0").unwrap(),
                            Arc::new(PluginPackage::from(HttpPlugin {})),
                        ),
                    ]),
                    ..Default::default()
                }
            },
        }
    }
}

impl Into<PolywrapClientConfig> for SystemClientConfig {
    fn into(self) -> PolywrapClientConfig {
        self.inner_config
    }
}

impl Into<ClientConfig> for SystemClientConfig {
    fn into(self) -> ClientConfig {
        self.inner_config.into()
    }
}
