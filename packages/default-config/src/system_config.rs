use polywrap_client_builder::PolywrapClientConfig;
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_http_plugin::HttpPlugin;

use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    package::WrapPackage,
    uri::Uri,
    wrapper::Wrapper,
};
use polywrap_plugin::package::PluginPackage;
use std::{collections::HashMap, sync::Arc};

use crate::embeds::{fs_resolver, http_resolver};

pub struct SystemClientConfig {
    polywrap_client_config: PolywrapClientConfig,
}

impl Default for SystemClientConfig {
    fn default() -> Self {
        Self {
            polywrap_client_config: {
                let mut interfaces = HashMap::new();
                interfaces.insert(
                    "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                    vec![
                        Uri::try_from("ens/wraps.eth:http-uri-resolver-ext@1.0.1").unwrap(),
                        Uri::try_from("ens/wraps.eth:file-system-uri-resolver-ext@1.0.1").unwrap(),
                    ],
                );

                let redirects: Vec<UriRedirect> = vec![
                    UriRedirect {
                        from: Uri::try_from("ens/wraps.eth:http@1.1.0").unwrap(),
                        to: Uri::try_from("plugin/http@1.1.0").unwrap(),
                    },
                    UriRedirect {
                        from: Uri::try_from("wrap://ens/wraps.eth:file-system@1.0.0").unwrap(),
                        to: Uri::try_from("plugin/file-system@1.0.0").unwrap(),
                    },
                ];

                PolywrapClientConfig {
                    interfaces: Some(interfaces),
                    wrappers: Some(get_default_wrappers()),
                    packages: Some(get_default_plugins()),
                    redirects: Some(redirects),
                    ..Default::default()
                }
            },
        }
    }
}

impl Into<PolywrapClientConfig> for SystemClientConfig {
    fn into(self) -> PolywrapClientConfig {
        self.polywrap_client_config
    }
}

impl Into<ClientConfig> for SystemClientConfig {
    fn into(self) -> ClientConfig {
        self.polywrap_client_config.into()
    }
}

pub fn get_default_wrappers() -> Vec<(Uri, Arc<dyn Wrapper>)> {
    let fs_resolver_package = Arc::new(fs_resolver::wasm_wrapper());
    let http_resolver_package = Arc::new(http_resolver::wasm_wrapper());

    vec![
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
