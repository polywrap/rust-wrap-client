use std::{sync::Arc, collections::HashMap};
use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_resolvers::{uri_resolver_wrapper::UriResolverWrapper, extendable_uri_resolver::ExtendableUriResolver};
use futures::lock::Mutex;

use polywrap_core::{uri::Uri, resolvers::{uri_resolution_context::{UriPackage, UriResolutionContext, UriPackageOrWrapper}, recursive_resolver::RecursiveResolver, uri_resolver_like::UriResolverLike, uri_resolver::UriResolver}, client::ClientConfig, interface_implementation::InterfaceImplementations};
use polywrap_core::resolvers::{static_resolver::{StaticResolver, StaticResolverLike}, resolver_with_history::ResolverWithHistory};
use polywrap_plugin::package::PluginPackage;
use polywrap_tests_utils::helpers::get_tests_path;
use fs_resolver_plugin::FileSystemResolverPlugin;
use filesystem_plugin::FileSystemPlugin;
use http_resolver_plugin::HttpResolverPlugin;
use http_plugin::HttpPlugin;
use serde_json::Value;

#[tokio::test]
async fn test_uri_resolver_wrapper() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let wrapper_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);
    let wrapper_uri = Uri::try_from(format!("fs/{}", wrapper_path)).unwrap();

    let fs = FileSystemPlugin {
        env: Value::Null
    };
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin {
        env: Value::Null
    };
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let resolver = StaticResolver::from(
        vec![
            StaticResolverLike::Package(UriPackage {
                uri: Uri::try_from("wrap://ens/fs.polywrap.eth").unwrap(),
                package: fs_package,
            }),
            StaticResolverLike::Package(UriPackage {
                uri: Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
                package: fs_resolver_package,
            }),
        ]
    );

    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: None,
        resolver: Arc::new(resolver)
    });

    let mut uri_resolution_context = UriResolutionContext::new();
    let uri_resolver_wrapper = UriResolverWrapper::new(
        Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap()
    );
    let result = uri_resolver_wrapper._try_resolve_uri(
        &wrapper_uri.clone(), 
        &client.loader, 
        &mut uri_resolution_context
    ).await;

    if let Ok(UriPackageOrWrapper::Wrapper(_, _w)) = result {
        dbg!("works :)");
    }
    
}

#[tokio::test]
async fn recursive_uri_resolver() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let wrapper_local_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);
    let _fs_wrapper_uri = Uri::try_from(format!("fs/{}", wrapper_local_path)).unwrap();

    let wrapper_github_path = "https://raw.githubusercontent.com/polywrap/wasm-test-harness/v0.2.1/wrappers/subinvoke/00-subinvoke/implementations/as";
    let http_wrapper_uri = Uri::try_from(format!("http/{}", wrapper_github_path)).unwrap();

    let fs = FileSystemPlugin {
        env: Value::Null
    };
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin {
        env: Value::Null
    };
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let http = HttpPlugin {
        env: Value::Null
    };
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(Mutex::new(http_plugin_package));

    let http_resolver = HttpResolverPlugin {
        env: Value::Null
    };
    let http_resolver_plugin_package: PluginPackage = http_resolver.into();
    let http_resolver_package = Arc::new(Mutex::new(http_resolver_plugin_package));

    let static_resolver = StaticResolver::from(
        vec![
            StaticResolverLike::Package(UriPackage {
                uri: Uri::try_from("wrap://ens/fs.polywrap.eth").unwrap(),
                package: fs_package,
            }),
            StaticResolverLike::Package(UriPackage {
                uri: Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
                package: fs_resolver_package,
            }),
            StaticResolverLike::Package(UriPackage {
                uri: Uri::try_from("wrap://ens/http.polywrap.eth").unwrap(),
                package: http_package,
            }),
            StaticResolverLike::Package(UriPackage {
                uri: Uri::try_from("wrap://ens/http-resolver.polywrap.eth").unwrap(),
                package: http_resolver_package,
            }),
            
        ]
    );

    let mut interfaces: InterfaceImplementations = HashMap::new();
    interfaces.insert(
        "wrap://ens/uri-resolver.core.polywrap.eth".to_string(), 
        vec![
            Uri::try_from("wrap://ens/http-resolver.polywrap.eth").unwrap(),
            Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
        ]
    );

    let extendable_uri_resolver = ExtendableUriResolver::new(None);
    let extendable_resolver_like = UriResolverLike::Resolver(Box::new(extendable_uri_resolver));
    let static_resolver_like = UriResolverLike::Resolver(Box::new(static_resolver));
    let recursive_resolver = RecursiveResolver::from(
        vec![static_resolver_like, extendable_resolver_like]
    );

    let r = Arc::new(recursive_resolver);

    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: Some(interfaces),
        resolver: r.clone()
    });
    let mut uri_resolution_context = UriResolutionContext::new();
    let result = r.try_resolve_uri(
        &http_wrapper_uri.clone(), 
        &client.loader, 
        &mut uri_resolution_context
    ).await;

    if let Ok(r) = result {
        if let UriPackageOrWrapper::Wrapper(_, _w) = r {
            dbg!("works");
        }
    } else {
      println!("{:?}", result.err().unwrap());
    }
    // assert_eq!(true, false);
}