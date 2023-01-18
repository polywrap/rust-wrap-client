use std::collections::HashMap;
use std::{sync::Arc};

use futures::{executor::block_on};
use jni::JNIEnv;
use jni::sys::jstring;
use jni::{sys::{jlong}, objects::{JClass, JString}};
pub use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::interface_implementation::InterfaceImplementations;
use polywrap_core::resolvers::recursive_resolver::RecursiveResolver;
pub use polywrap_core::resolvers::static_resolver::StaticResolver;
use polywrap_core::{
    client::ClientConfig,
    invoke::{InvokeArgs, Invoker},
    uri::Uri,
};
use android_logger::Config;
use log::Level;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_nStartLogger(
    _: JNIEnv,
    _: JClass,
) {
  android_logger::init_once(
    Config::default().with_min_level(Level::Trace));
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_nCreateClient(
  _: JNIEnv,
  _: JClass,
  resolver_ptr: jlong,
) -> *mut PolywrapClient {
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
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_nDestructClient(
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
pub extern "system" fn Java_com_example_polywrapmobile_NativeClient_nInvoke(
    env: JNIEnv,
    _: JClass,
    client_ptr: jlong,
    uri: JString,
    method: JString,
    args: JString,
) -> jstring {
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

    let args: String = env
      .get_string(args)
      .expect("Couldn't get java string! for args")
      .into();

    let uri: Uri = uri.try_into().unwrap();
    let json_args: serde_json::Value = serde_json::from_str(&args).unwrap();

    let invoke_args = InvokeArgs::UIntArray(polywrap_msgpack::serialize(json_args).unwrap());

    let invoke_result = block_on(async {
        client
            .invoke_raw(&uri, &method, Some(&invoke_args), None, None)
            .await
            .unwrap()
    });

    let output = env
    .new_string(format!("Result: {:#?}", invoke_result))
    .expect("Couldn't create java string!");
    output.into_raw()
}
