use std::{sync::Arc};

use filesystem_plugin::FileSystemPlugin;
use fs_resolver_plugin::FileSystemResolverPlugin;
use futures::{executor::block_on, lock::Mutex};
use jni::JNIEnv;
use jni::sys::jstring;
use jni::{sys::{jlong}, objects::{JClass, JString}};
use logger::Logger;
pub use polywrap_client::polywrap_client::PolywrapClient;
pub use polywrap_core::resolvers::static_resolver::StaticResolver;
use polywrap_core::{
    client::ClientConfig,
    invoke::{InvokeArgs, Invoker},
    resolvers::{static_resolver::StaticResolverLike, uri_resolution_context::UriPackage},
    uri::Uri,
};
use polywrap_plugin::package::PluginPackage;

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

    let resolver = StaticResolver::from(vec![
        StaticResolverLike::Package(UriPackage {
            uri: Uri::try_from("wrap://ens/fs.polywrap.eth").unwrap(),
            package: fs_package,
        }),
        StaticResolverLike::Package(UriPackage {
            uri: Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
            package: fs_resolver_package,
        }),
    ]);

    Box::into_raw(Box::new(resolver)) as jlong
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_destructResolver(
    _: JNIEnv,
    _: JClass,
    resolver_ptr: jlong,
) {
  unsafe {
    drop(Box::from_raw(resolver_ptr as *mut StaticResolver));
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
        Arc::from_raw(resolver_ptr as *mut StaticResolver)
    };

    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: None,
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
