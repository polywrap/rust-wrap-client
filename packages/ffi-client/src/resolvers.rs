use std::sync::Arc;

use filesystem_plugin::FileSystemPlugin;
use fs_resolver_plugin::FileSystemResolverPlugin;
use futures::lock::Mutex;
use http_plugin::HttpPlugin;
use http_resolver_plugin::HttpResolverPlugin;
use jni::objects::{JMap, JObject};
use jni::JNIEnv;
use jni::{objects::JClass, sys::jlong};
pub use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::client::UriRedirect;
use polywrap_core::resolvers::recursive_resolver::RecursiveResolver;
pub use polywrap_core::resolvers::static_resolver::StaticResolver;
use polywrap_core::resolvers::uri_resolver_like::UriResolverLike;
use polywrap_core::{
    resolvers::{static_resolver::StaticResolverLike, uri_resolution_context::UriPackage},
    uri::Uri,
};
use polywrap_plugin::package::PluginPackage;
use polywrap_resolvers::extendable_uri_resolver::ExtendableUriResolver;
use serde_json::json;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_nCreateResolver(
    env: JNIEnv,
    _: JClass,
    redirects: JObject,
) -> jlong {
    let fs = FileSystemPlugin { env: json!({}) };
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin { env: json!({}) };
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let http = HttpPlugin { env: json!({}) };
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(Mutex::new(http_plugin_package));

    let http_resolver = HttpResolverPlugin { env: json!({}) };
    let http_resolver_plugin_package: PluginPackage = http_resolver.into();
    let http_resolver_package = Arc::new(Mutex::new(http_resolver_plugin_package));

    let redirects_map: JMap = env
        .get_map(redirects)
        .expect("Couldn't get java Map! for Redirects")
        .into();

    let mut static_resolver_likes = vec![
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
    ];

    redirects_map
        .iter()
        .and_then(|mut iter| {
            iter.try_for_each(|(from, to)| {
                let from = env.get_string(from.into()).map(|s| String::from(s))?;

                let to = env.get_string(to.into()).map(|s| String::from(s))?;

                static_resolver_likes.push(StaticResolverLike::Redirect(UriRedirect {
                    from: from.try_into().unwrap(),
                    to: to.try_into().unwrap(),
                }));

                Ok(())
            })
        })
        .unwrap();

    let static_resolver =
        StaticResolver::from(static_resolver_likes);

    let extendable_uri_resolver = ExtendableUriResolver::new(None);
    let extendable_resolver_like = UriResolverLike::Resolver(Box::new(extendable_uri_resolver));
    let static_resolver_like = UriResolverLike::Resolver(Box::new(static_resolver));
    let recursive_resolver =
        RecursiveResolver::from(vec![static_resolver_like, extendable_resolver_like]);

    Box::into_raw(Box::new(recursive_resolver)) as jlong
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_nDestructResolver(
    _: JNIEnv,
    _: JClass,
    resolver_ptr: jlong,
) {
    unsafe {
        drop(Box::from_raw(resolver_ptr as *mut RecursiveResolver));
    };
}
