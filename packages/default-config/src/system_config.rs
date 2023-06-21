use polywrap_client_builder::PolywrapClientConfig;
use polywrap_core_macros::uri;
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_http_plugin::HttpPlugin;

use polywrap_core::{client::ClientConfig, uri::Uri};
use polywrap_plugin::package::PluginPackage;
use std::{collections::HashMap, sync::Arc};

use crate::embeds::{fs_resolver, http_resolver};

pub struct SystemClientConfig(PolywrapClientConfig);

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
                ])),
                interfaces: Some(HashMap::from([(
                    "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                    vec![
                        uri!("ens/wraps.eth:http-uri-resolver-ext@1.0.1"),
                        uri!("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1"),
                    ],
                )])),
                wrappers: Some(vec![
                    (
                        uri!("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1"),
                        Arc::new(fs_resolver::wasm_wrapper()),
                    ),
                    (
                        uri!("ens/wraps.eth:http-uri-resolver-ext@1.0.1"),
                        Arc::new(http_resolver::wasm_wrapper()),
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
