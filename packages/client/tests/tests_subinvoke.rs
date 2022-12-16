use filesystem_plugin::FileSystemPlugin;
use fs_resolver_plugin::FileSystemResolverPlugin;
use futures::lock::Mutex;
use http_plugin::HttpPlugin;
use http_resolver_plugin::HttpResolverPlugin;
use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::interface_implementation::InterfaceImplementations;
use polywrap_core::invoke::Invoker;
use polywrap_core::resolvers::recursive_resolver::RecursiveResolver;
use polywrap_core::resolvers::static_resolver::{StaticResolver, StaticResolverLike};
use polywrap_core::resolvers::uri_resolution_context::UriPackage;
use polywrap_core::resolvers::uri_resolver_like::UriResolverLike;
use polywrap_core::{client::UriRedirect, invoke::InvokeArgs, uri::Uri};
use polywrap_msgpack::msgpack;
use polywrap_plugin::package::PluginPackage;
use polywrap_resolvers::extendable_uri_resolver::ExtendableUriResolver;
use polywrap_resolvers::legacy::{base::BaseResolver, filesystem::FilesystemResolver};
use polywrap_tests_utils::helpers::get_tests_path;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
async fn subinvoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let subinvoke_uri: Uri = format!("fs/{}/subinvoke/00-subinvoke/implementations/as", path)
        .try_into()
        .unwrap();
    let invoke_uri: Uri = format!("fs/{}/subinvoke/01-invoke/implementations/as", path)
        .try_into()
        .unwrap();

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let redirect = UriRedirect::new("ens/add.eth".try_into().unwrap(), subinvoke_uri.clone());

    let redirects_static_like = StaticResolverLike::Redirect(redirect);
    let static_resolver = StaticResolver::from(vec![redirects_static_like]);

    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: None,
        resolver: Arc::new(BaseResolver::new(
            Box::new(fs_resolver),
            Box::new(static_resolver),
        )),
    });

    let invoke_args = InvokeArgs::Msgpack(msgpack!({"a": 1, "b": 1}));

    let invoke_result = client
        .invoke_and_decode::<u32>(&subinvoke_uri, "add", Some(&invoke_args), None, None)
        .await
        .unwrap();

    assert_eq!(invoke_result, 2);
}

#[tokio::test]
async fn test() {
    let fs = FileSystemPlugin { env: Value::Null };
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin { env: Value::Null };
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let http = HttpPlugin { env: Value::Null };
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(Mutex::new(http_plugin_package));

    let http_resolver = HttpResolverPlugin { env: Value::Null };
    let http_resolver_plugin_package: PluginPackage = http_resolver.into();
    let http_resolver_package = Arc::new(Mutex::new(http_resolver_plugin_package));

    let static_resolver = StaticResolver::from(vec![
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
        StaticResolverLike::Redirect(UriRedirect {
          from: Uri::try_from("wrap://ens/add.eth").unwrap(),
          to: Uri::try_from("wrap://http/https://raw.githubusercontent.com/namesty/test-wrappers/main/subinvoke").unwrap()
        })
    ]);

    let extendable_uri_resolver = ExtendableUriResolver::new(None);
    let extendable_resolver_like = UriResolverLike::Resolver(Box::new(extendable_uri_resolver));
    let static_resolver_like = UriResolverLike::Resolver(Box::new(static_resolver));
    let recursive_resolver =
        RecursiveResolver::from(vec![static_resolver_like, extendable_resolver_like]);

    let resolver = Arc::new(recursive_resolver);

    let mut interfaces: InterfaceImplementations = HashMap::new();
    interfaces.insert(
        "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
        vec![
            Uri::try_from("wrap://ens/http-resolver.polywrap.eth").unwrap(),
            Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
        ],
    );

    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: Some(interfaces),
        resolver,
    });

    let uri = Uri::try_from("wrap://ens/add.eth").unwrap();
    let args_json = String::from("{\"a\": 1, \"b\": 1}");
    let json_args: Value = serde_json::from_str(&args_json).unwrap();

    let invoke_args = InvokeArgs::UIntArray(polywrap_msgpack::serialize(json_args).unwrap());
    let result = client.invoke(&uri, "add", Some(&invoke_args), None, None).await.unwrap();

    println!("{:?}", result);
}
