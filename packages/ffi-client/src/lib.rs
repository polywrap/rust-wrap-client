use std::collections::HashMap;
use std::{sync::Arc};

use filesystem_plugin::FileSystemPlugin;
use fs_resolver_plugin::FileSystemResolverPlugin;
use futures::{executor::block_on, lock::Mutex};
use http_plugin::HttpPlugin;
use http_resolver_plugin::HttpResolverPlugin;
use jni::JNIEnv;
use jni::sys::jstring;
use jni::{sys::{jlong}, objects::{JClass, JString}};
use logger::Logger;
pub use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::client::UriRedirect;
use polywrap_core::interface_implementation::InterfaceImplementations;
use polywrap_core::resolvers::recursive_resolver::RecursiveResolver;
pub use polywrap_core::resolvers::static_resolver::StaticResolver;
use polywrap_core::resolvers::uri_resolver_like::UriResolverLike;
use polywrap_core::{
    client::ClientConfig,
    invoke::{InvokeArgs, Invoker},
    resolvers::{static_resolver::StaticResolverLike, uri_resolution_context::UriPackage},
    uri::Uri,
};
use polywrap_plugin::package::PluginPackage;
use polywrap_resolvers::extendable_uri_resolver::ExtendableUriResolver;

pub mod logger;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_createResolver(
    env: JNIEnv,
    _: JClass,
) -> jlong {
    let logger = Logger::new(env, "FFIPolywrapClient").unwrap();
    logger.d("Invoked 'Java_com_example_polywrapmobile_NativeClient_createResolver'").unwrap();

    let fs = FileSystemPlugin {};
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin {};
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let http = HttpPlugin {};
    let http_plugin_package: PluginPackage = http.into();
    let http_package = Arc::new(Mutex::new(http_plugin_package));

    let http_resolver = HttpResolverPlugin {};
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

    Box::into_raw(Box::new(recursive_resolver)) as jlong
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_destructResolver(
    _: JNIEnv,
    _: JClass,
    resolver_ptr: jlong,
) {
  unsafe {
    drop(Box::from_raw(resolver_ptr as *mut RecursiveResolver));
  };
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_createClient(
  env: JNIEnv,
  _: JClass,
  resolver_ptr: jlong,
) -> *mut PolywrapClient {
    let logger = Logger::new(env, "FFIPolywrapClient").unwrap();
    logger.d("Invoked 'Java_com_example_polywrapmobile_NativeClient_createClient'").unwrap();

    let resolver = unsafe {
        Arc::from_raw(resolver_ptr as *mut RecursiveResolver)
    };

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

    Box::into_raw(Box::new(client))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_destructClient(
    _: JNIEnv,
    _: JClass,
    client_ptr: jlong,
) {
  unsafe {
    drop(Box::from_raw(client_ptr as *mut PolywrapClient));
  };
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_invoke(
    env: JNIEnv,
    _: JClass,
    client_ptr: jlong,
    uri: JString,
    method: JString,
    args_ptr: jlong,
    args_len: jlong,
) -> jstring {
    let logger = Logger::new(env, "FFIPolywrapClient").unwrap();
    logger.d("Invoked 'Java_com_example_polywrapmobile_NativeClient_invoke'").unwrap();

    let client = unsafe {
        Box::from_raw(client_ptr as *mut PolywrapClient)
    };

    let uri_str: String = env
      .get_string(uri)
      .expect("Couldn't get java string! for URI")
      .into();

    let uri: Uri = uri_str.try_into().unwrap();

    let method: String = env
      .get_string(method)
      .expect("Couldn't get java string! for Method")
      .into();

    let args = unsafe {
        let len = args_len as usize;
        Vec::from_raw_parts(args_ptr as *mut u8, len, len)
    };

    let uri: Uri = uri.try_into().unwrap();

    let invoke_result = block_on(async {
        client
            .invoke(&uri, &method, Some(&InvokeArgs::UIntArray(args)), None, None)
            .await
            .unwrap()
    });

    let output = env
    .new_string(format!("Result: {:#?}", invoke_result))
    .expect("Couldn't create java string!");
    output.into_raw()
}
