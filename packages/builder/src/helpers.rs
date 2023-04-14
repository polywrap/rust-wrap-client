use std::{sync::{Arc, Mutex}, collections::HashMap};
use fs_resolver_plugin::FileSystemResolverPlugin;

use filesystem_plugin::FileSystemPlugin;
use http_plugin::HttpPlugin;
use http_resolver_plugin::HttpResolverPlugin;
use polywrap_core::{resolvers::{uri_resolution_context::UriPackage, static_resolver::{StaticResolverLike, StaticResolver}, uri_resolver_like::UriResolverLike, recursive_resolver::RecursiveResolver, uri_resolver::UriResolver}, uri::Uri, client::ClientConfig};
use polywrap_plugin::package::PluginPackage;
use polywrap_resolvers::extendable_uri_resolver::ExtendableUriResolver;
use serde_json::{Value};
use polywrap_core::client::UriRedirect;
use polywrap_core::env::Envs;


use crate::types::BuilderConfig;

pub fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

pub fn add_default() -> BuilderConfig {
    let mut interfaces = HashMap::new();
    interfaces.insert(
        "wrap://ens/uri-resolver.core.polywrap.eth".to_string(), 
        vec![
            // Uri::try_from("ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.0").unwrap(),
            Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
            Uri::try_from("wrap://ens/http-resolver.polywrap.eth").unwrap(),
        ]
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
        to: Uri::try_from("wrap://ens/http.polywrap.eth").unwrap()
    });

    BuilderConfig { 
        interfaces: Some(interfaces),
        envs: Some(envs),
        wrappers: None,
        packages: Some(get_default_plugins()),
        redirects: Some(redirects),
        resolvers: None
    }
}

pub fn get_default_plugins() -> Vec<UriPackage> {
    let fs = FileSystemPlugin { };
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin { };
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let http = HttpPlugin { };
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(Mutex::new(http_plugin_package));

    let http_resolver = HttpResolverPlugin { };
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
            package: fs_package
        },
        UriPackage {
            uri: Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
            package: fs_resolver_package
        },
        UriPackage {
            uri: Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
            package: http_package
        },
        UriPackage {
            uri: Uri::try_from("wrap://ens/http-resolver.polywrap.eth").unwrap(),
            package: http_resolver_package
        },
    ]
}

pub fn build_resolver(builder: BuilderConfig) -> ClientConfig {
    let mut static_resolvers: Vec<StaticResolverLike> = vec![];

    if let Some(wrappers) = builder.wrappers {
        for w in wrappers {
            static_resolvers.push(StaticResolverLike::Wrapper(w));
        };
    }

    if let Some(packages) = builder.packages {
        for p in packages {
            static_resolvers.push(StaticResolverLike::Package(p));
        };
    }

    if let Some(redirects) = builder.redirects {
        for r in redirects {
            static_resolvers.push(StaticResolverLike::Redirect(r));
        };
    }
    let static_resolver = StaticResolver::from(static_resolvers);
    let extendable_resolver = ExtendableUriResolver::new(None);
    
    let resolvers = vec![
        UriResolverLike::Resolver(Box::new(static_resolver)),
        UriResolverLike::Resolver(Box::new(extendable_resolver)),
    ];
    
    ClientConfig {
        envs: builder.envs.clone(),
        interfaces: builder.interfaces.clone(),
        resolver: Arc::new(
            RecursiveResolver::from(resolvers)
        ) as Arc<dyn UriResolver>
    }
}