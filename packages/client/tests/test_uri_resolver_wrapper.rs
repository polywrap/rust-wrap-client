use std::sync::Arc;
use polywrap_client::polywrap_client::PolywrapClient;
use tokio::sync::Mutex;

use polywrap_core::{uri::Uri, uri_resolution_context::{UriPackage, UriResolutionContext, UriPackageOrWrapper}, client::ClientConfig};
use polywrap_resolvers::{uri_resolver_wrapper::UriResolverWrapper, static_resolver::{StaticResolver, StaticResolverLike}, resolver_with_history::ResolverWithHistory};
use polywrap_plugin::package::PluginPackage;
use polywrap_tests_utils::helpers::get_tests_path;
use fs_resolver_plugin::FileSystemResolverPlugin;
use filesystem_plugin::FileSystemPlugin;

#[tokio::test]
async fn test_uri_resolver_wrapper() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let wrapper_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);
    let wrapper_uri = Uri::try_from(format!("fs/{}", wrapper_path)).unwrap();

    let fs = FileSystemPlugin {};
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin {};
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
        resolver: Arc::new(Mutex::new(Box::new(resolver)))
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

    if let Ok(r) = result {
        if let UriPackageOrWrapper::Wrapper(_, _w) = r {
            dbg!("works :)");
        }
    }
    
}