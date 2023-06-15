use std::{sync::Arc, collections::HashMap};

use polywrap_client::{client::PolywrapClient, builder::types::BuilderConfig, plugin::package::PluginPackage};
use polywrap_core::{client::{ClientConfig, UriRedirect}, resolution::uri_resolver::UriResolver, uri::Uri, file_reader::SimpleFileReader};
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_msgpack::msgpack;
use polywrap_resolver_extensions::extendable_uri_resolver::ExtendableUriResolver;
use polywrap_resolvers::{recursive_resolver::RecursiveResolver, resolution_result_cache_resolver::ResolutionResultCacheResolver, resolver_vec, static_resolver::{StaticResolver, StaticResolverLike}};
use polywrap_wasm::wasm_wrapper::WasmWrapper;

pub fn get_client_with_module(module: &[u8]) -> PolywrapClient {
    let config = {
        let redirects = Some(vec![
            UriRedirect {
                from: Uri::try_from("ens/wraps.eth:http@1.1.0").unwrap(),
                to: Uri::try_from("plugin/http@1.1.0").unwrap(),
            },
        ]);

        let mut interfaces = HashMap::new();
        interfaces.insert(
            "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
            vec![
                Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(),
            ],
        );

        interfaces.insert(
            "wrap://ens/wraps.eth:ipfs-http-client@1.0.0".to_string(),
            vec![Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap()],
        );

        let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
        envs.insert(
            "ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1".to_string(),
            msgpack!({
                "provider": "https://ipfs.wrappers.io",
                "fallbackProviders": ["https://ipfs.io"],
                "retries": { "tryResolveUri": 2, "getFile": 2 },
                "disableParallelRequests": true,
            }),
        );

        let fs = FileSystemPlugin {};
        let fs_plugin_package: PluginPackage = fs.into();
        let fs_package = Arc::new(fs_plugin_package);

        BuilderConfig {
            interfaces: Some(interfaces),
            envs: Some(envs),
            wrappers: Some(vec![
                (
                    Uri::try_from("wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(),
                    Arc::new(WasmWrapper::new(module.to_vec(), Arc::new(SimpleFileReader::new()))),
                ),
                // (
                //     Uri::try_from("wrap://ens/wraps.eth:ipfs-http-client@1.0.0").unwrap(),
                //     Arc::new(WasmWrapper::new(ipfs_http_client::wrap_wasm().unwrap(), Arc::new(SimpleFileReader::new()))),
                // ),
            ]),
            packages: Some(vec![
                (
                    Uri::try_from("plugin/file-system@1.0.0").unwrap(),
                    fs_package,
                ),
            ]),
            redirects,
            resolvers: None,
        }
    };

    let mut static_resolvers: Vec<StaticResolverLike> = config.wrappers.unwrap().iter().map(|(uri, wrapper)| {
        StaticResolverLike::Wrapper(uri.clone(), wrapper.clone())
    }).collect::<Vec<StaticResolverLike>>();

    static_resolvers.append(&mut config.packages.unwrap().iter().map(|(uri, package)| {
        StaticResolverLike::Package(uri.clone(), package.clone())
    }).collect::<Vec<StaticResolverLike>>());
    
    let config = ClientConfig {
        envs: config.envs.clone(),
        interfaces: config.interfaces.clone(),
        resolver: Arc::new(RecursiveResolver::from(
            Box::from(ResolutionResultCacheResolver::from(resolver_vec![
                StaticResolver::from(static_resolvers),
                ExtendableUriResolver::new(None),
            ])) as Box<dyn UriResolver>
        )),
    };

    let client = PolywrapClient::new(config);

    client
}