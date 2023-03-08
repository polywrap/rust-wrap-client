use std::{ffi::{CStr}, sync::{Arc, Mutex}, collections::HashMap};
use filesystem_plugin::FileSystemPlugin;
use fs_resolver_plugin::FileSystemResolverPlugin;
use http_plugin::HttpPlugin;
use http_resolver_plugin::HttpResolverPlugin;
use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    invoke::{Invoker},
    uri::Uri,
    resolvers::{recursive_resolver::RecursiveResolver, uri_resolution_context::UriPackage, static_resolver::{StaticResolverLike, StaticResolver}, uri_resolver_like::UriResolverLike},
    interface_implementation::InterfaceImplementations,
    client::ClientConfig
};
use polywrap_plugin::package::PluginPackage;
use polywrap_resolvers::extendable_uri_resolver::ExtendableUriResolver;
use serde_json::Value;

pub mod builder;

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: libc::c_int,
}

#[no_mangle]
pub extern "C" fn invoke(
    client_ptr: *const libc::c_char,
    uri: *const libc::c_char,
    method: *const libc::c_char,
    args: *const libc::c_char,
) -> Buffer {
    let client = unsafe {
        Box::from_raw(client_ptr as *mut PolywrapClient)
    };

    let uri_c_str = unsafe { CStr::from_ptr(uri) };
    let uri_str = match uri_c_str.to_str() {
        Ok(u) => u.to_string(),
        Err(_) => panic!("Couldn't get CStr for URI")
    };

    let method_c_str = unsafe { CStr::from_ptr(method) };
    let method_str = match method_c_str.to_str() {
        Ok(u) => u.to_string(),
        Err(_) => panic!("Couldn't get CStr for Method")
    };

    let args_c_str = unsafe { CStr::from_ptr(args) };
    let args_str = match args_c_str.to_str() {
        Ok(u) => u.to_string(),
        Err(_) => panic!("Couldn't get CStr for Args")
    };

    let json_args: serde_json::Value = serde_json::from_str(&args_str).unwrap();
    let invoke_args = polywrap_msgpack::serialize(json_args).unwrap();

    let mut invoke_result =
        client
            .invoke_raw(&uri_str.try_into().unwrap(), &method_str, Some(&invoke_args), None, None)
            .unwrap().into_boxed_slice();

    let buffer = Buffer {
        data: invoke_result.as_mut_ptr(),
        len: invoke_result.len() as i32
    };
    std::mem::forget(invoke_result);

    buffer
}

#[no_mangle]
pub extern "C" fn create_client(
    resolver_ptr: *const libc::c_char
) -> *const libc::c_char {
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

    Box::into_raw(Box::new(client)) as *const libc::c_char
}

#[no_mangle]
pub extern "C" fn create_resolver() -> *const libc::c_char {
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

    let static_resolver_likes = vec![
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
    let static_resolver = StaticResolver::from(static_resolver_likes);
    let extendable_uri_resolver = ExtendableUriResolver::new(None);
    let extendable_resolver_like = UriResolverLike::Resolver(Box::new(extendable_uri_resolver));
    let static_resolver_like = UriResolverLike::Resolver(Box::new(static_resolver));
    let recursive_resolver = RecursiveResolver::from(vec![static_resolver_like, extendable_resolver_like]);

    Box::into_raw(Box::new(recursive_resolver)) as *const libc::c_char
}
