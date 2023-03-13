use std::{ffi::c_char, sync::{Mutex, Arc}};

use libc::c_void;
use polywrap_client::{
    builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler},
    core::{
        env::Env,
        resolvers::{
            uri_resolution_context::{UriPackage, UriWrapper},
        },
        uri::Uri,
    },
};
use polywrap_plugin::{wrapper::PluginWrapper, package::PluginPackage};
use polywrap_wasm::{wasm_wrapper::WasmWrapper, wasm_package::WasmPackage};

use crate::{utils::{
    get_string_from_cstr_ptr, instantiate_from_ptr, instantiate_from_ptr_and_take_ownership,
    into_raw_ptr_and_forget,
}, resolvers::uri_resolver_like::SafeUriResolverLikeVariant};

#[no_mangle]
pub extern "C" fn new_builder_config() -> *mut c_void {
    let builder_config = BuilderConfig::new(None);

    into_raw_ptr_and_forget(builder_config) as *mut c_void
}

#[no_mangle]
pub extern "C" fn add_env(builder_config_ptr: *mut BuilderConfig, uri: *const c_char, env: *const c_char) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);

    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();
    let env: Env = serde_json::from_str(&get_string_from_cstr_ptr(env)).unwrap();

    builder.add_env(uri, env);
}

#[no_mangle]
pub extern "C" fn remove_env(builder_config_ptr: *mut BuilderConfig, uri: *const c_char) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    builder.remove_env(uri);
}

#[no_mangle]
pub extern "C" fn set_env(builder_config_ptr: *mut BuilderConfig, uri: *const c_char, env: *const c_char) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);

    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();
    let env: Env = serde_json::from_str(&get_string_from_cstr_ptr(env)).unwrap();

    builder.set_env(uri, env);
}

#[no_mangle]
pub extern "C" fn add_interface_implementation(
    builder_config_ptr: *mut BuilderConfig,
    interface_uri: *const c_char,
    implementation_uri: *const c_char,
) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);

    let interface_uri: Uri = get_string_from_cstr_ptr(interface_uri).try_into().unwrap();
    let implementation_uri: Uri = get_string_from_cstr_ptr(implementation_uri).try_into().unwrap();

    builder.add_interface_implementation(interface_uri, implementation_uri);
}

#[no_mangle]
pub extern "C" fn remove_interface_implementation(
    builder_config_ptr: *mut BuilderConfig,
    interface_uri: *const c_char,
    implementation_uri: *const c_char,
) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);

    let interface_uri: Uri = get_string_from_cstr_ptr(interface_uri).try_into().unwrap();
    let implementation_uri: Uri = get_string_from_cstr_ptr(implementation_uri).try_into().unwrap();

    builder.remove_interface_implementation(interface_uri, implementation_uri);
}

#[no_mangle]
pub extern "C" fn add_wasm_wrapper(builder_config_ptr: *mut BuilderConfig, uri: *const c_char, wrapper: *mut WasmWrapper) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let wrapper = Arc::new(Mutex::new(instantiate_from_ptr(wrapper)));
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    let uri_wrapper = UriWrapper {
      uri,
      wrapper
    };

    builder.add_wrapper(uri_wrapper);
}

#[no_mangle]
pub extern "C" fn add_plugin_wrapper(builder_config_ptr: *mut BuilderConfig, uri: *const c_char, wrapper: *mut PluginWrapper) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let wrapper = Arc::new(Mutex::new(instantiate_from_ptr(wrapper)));
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    let uri_wrapper = UriWrapper {
      uri,
      wrapper
    };

    builder.add_wrapper(uri_wrapper);
}

#[no_mangle]
pub extern "C" fn remove_wrapper(builder_config_ptr: *mut BuilderConfig, uri: *const c_char) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    builder.remove_wrapper(uri);
}

#[no_mangle]
pub extern "C" fn add_wasm_package(builder_config_ptr: *mut BuilderConfig, uri: *const c_char, package: *mut WasmPackage) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let package = Arc::new(Mutex::new(instantiate_from_ptr(package)));
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();
    
    let uri_package = UriPackage {
      uri,
      package
    };

    builder.add_package(uri_package);
}

#[no_mangle]
pub extern "C" fn add_plugin_package(builder_config_ptr: *mut BuilderConfig, uri: *const c_char, package: *mut PluginPackage) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let package = Arc::new(Mutex::new(instantiate_from_ptr(package)));
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    let uri_package = UriPackage {
      uri,
      package
    };

    builder.add_package(uri_package);
}

#[no_mangle]
pub extern "C" fn remove_package(builder_config_ptr: *mut BuilderConfig, uri: *const c_char) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    builder.remove_package(uri);
}

#[no_mangle]
pub extern "C" fn add_redirect(builder_config_ptr: *mut BuilderConfig, from: *const c_char, to: *const c_char) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);

    let from: Uri = get_string_from_cstr_ptr(from).try_into().unwrap();
    let to: Uri = get_string_from_cstr_ptr(to).try_into().unwrap();

    builder.add_redirect(from, to);
}

#[no_mangle]
pub extern "C" fn remove_redirect(builder_config_ptr: *mut BuilderConfig, from: *const c_char) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    let from: Uri = get_string_from_cstr_ptr(from).try_into().unwrap();

    builder.remove_redirect(from);
}

#[no_mangle]
pub extern "C" fn add_wrapper_resolver(builder_config_ptr: *mut BuilderConfig, resolver: SafeUriResolverLikeVariant) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    builder.add_resolver(resolver.into());
}

#[no_mangle]
pub extern "C" fn add_redirect_resolver(builder_config_ptr: *mut BuilderConfig, resolver: SafeUriResolverLikeVariant) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    builder.add_resolver(resolver.into());
}

#[no_mangle]
pub extern "C" fn add_package_resolver(builder_config_ptr: *mut BuilderConfig, resolver: SafeUriResolverLikeVariant) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    builder.add_resolver(resolver.into());
}

#[no_mangle]
pub extern "C" fn add_resolver(builder_config_ptr: *mut BuilderConfig, resolver: SafeUriResolverLikeVariant) {
    let mut builder = instantiate_from_ptr(builder_config_ptr);
    builder.add_resolver(resolver.into());
}

// Destroys builder_config
#[no_mangle]
pub extern "C" fn build_client(builder_config_ptr: *mut BuilderConfig) -> *const c_void {
    let builder = instantiate_from_ptr_and_take_ownership(builder_config_ptr);
    let config = builder.build();

    into_raw_ptr_and_forget(config)
}
