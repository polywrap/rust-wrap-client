use polywrap_client::{
    builder::types::{BuilderConfig as InnerBuilderConfig, ClientBuilder},
    core::{
        resolvers::uri_resolution_context::{UriPackage, UriWrapper},
        uri::Uri,
        wrapper::Wrapper,
    },
};
use polywrap_plugin::{module::PluginModule, package::PluginPackage, wrapper::PluginWrapper};
use polywrap_wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper};
use std::{
    ffi::{c_char, c_void},
    sync::{Arc, Mutex},
};

use crate::{
    ext_plugin::{ExtPluginModule, PluginInvokeFn},
    resolvers::uri_resolver_like::SafeUriResolverLikeVariant,
    utils::{get_string_from_cstr_ptr, instantiate_from_ptr},
};

pub struct BuilderConfigContainer {
    pub inner_builder: Mutex<InnerBuilderConfig>,
}

impl BuilderConfigContainer {
    pub fn new() -> BuilderConfigContainer {
        BuilderConfigContainer {
            inner_builder: Mutex::new(InnerBuilderConfig::new(None)),
        }
    }
}

pub fn new_builder_config() -> BuilderConfigContainer {
    BuilderConfigContainer::new()
}

pub fn add_env(builder: Arc<BuilderConfigContainer>, uri: &str, env: &str) {
    builder.inner_builder.lock().unwrap().add_env(
        uri.to_string().try_into().unwrap(),
        serde_json::from_str(env).unwrap(),
    );
}

pub fn remove_env(builder: Arc<BuilderConfigContainer>, uri: &str) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .remove_env(uri.to_string().try_into().unwrap());
}

pub fn set_env(builder: Arc<BuilderConfigContainer>, uri: &str, env: &str) {
    builder.inner_builder.lock().unwrap().set_env(
        uri.to_string().try_into().unwrap(),
        serde_json::from_str(env).unwrap(),
    );
}

pub fn add_interface_implementation(
    builder: Arc<BuilderConfigContainer>,
    interface_uri: &str,
    implementation_uri: &str,
) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .add_interface_implementation(
            interface_uri.try_into().unwrap(),
            implementation_uri.try_into().unwrap(),
        );
}

pub fn remove_interface_implementation(
    builder: Arc<BuilderConfigContainer>,
    interface_uri: &str,
    implementation_uri: &str,
) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .remove_interface_implementation(
            interface_uri.try_into().unwrap(),
            implementation_uri.try_into().unwrap(),
        );
}

pub fn add_wrapper(builder: Arc<BuilderConfigContainer>, uri: &str, wrapper: Box<dyn Wrapper>) {
    let uri_wrapper = UriWrapper {
        uri: uri.to_string().try_into().unwrap(),
        wrapper: Arc::new(Mutex::new(wrapper.type_id())),
    };

    builder.inner_builder.lock().unwrap().add_wrapper(wrapper)
}

#[no_mangle]
pub extern "C" fn add_wasm_wrapper(
    builder_config_ptr: *mut BuilderConfig,
    uri: *const c_char,
    wrapper: *mut WasmWrapper,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    let wrapper = Arc::new(Mutex::new(instantiate_from_ptr(wrapper)));
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    let uri_wrapper = UriWrapper { uri, wrapper };

    builder.add_wrapper(uri_wrapper);
}

#[no_mangle]
pub extern "C" fn add_plugin_wrapper(
    builder_config_ptr: *mut BuilderConfig,
    uri: *const c_char,
    plugin_ptr: *mut c_void,
    plugin_invoke_fn: PluginInvokeFn,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    let ext_plugin =
        Box::new(ExtPluginModule::new(plugin_ptr, plugin_invoke_fn)) as Box<dyn PluginModule>;
    let ext_plugin = Arc::new(Mutex::new(ext_plugin));
    let ext_plugin_wrapper = PluginWrapper::new(ext_plugin);

    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    let uri_wrapper = UriWrapper {
        uri,
        wrapper: Arc::new(Mutex::new(ext_plugin_wrapper)),
    };

    builder.add_wrapper(uri_wrapper);
}

#[no_mangle]
pub extern "C" fn remove_wrapper(builder_config_ptr: *mut BuilderConfig, uri: *const c_char) {
    let builder = unsafe { &mut *builder_config_ptr };
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    builder.remove_wrapper(uri);
}

#[no_mangle]
pub extern "C" fn add_wasm_package(
    builder_config_ptr: *mut BuilderConfig,
    uri: *const c_char,
    package: *mut WasmPackage,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    let package = Arc::new(Mutex::new(instantiate_from_ptr(package)));
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    let uri_package = UriPackage { uri, package };

    builder.add_package(uri_package);
}

#[no_mangle]
pub extern "C" fn add_plugin_package(
    builder_config_ptr: *mut BuilderConfig,
    uri: *const c_char,
    package: *mut PluginPackage,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    let package = Arc::new(Mutex::new(instantiate_from_ptr(package)));
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    let uri_package = UriPackage { uri, package };

    builder.add_package(uri_package);
}

#[no_mangle]
pub extern "C" fn remove_package(builder_config_ptr: *mut BuilderConfig, uri: *const c_char) {
    let builder = unsafe { &mut *builder_config_ptr };
    let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();

    builder.remove_package(uri);
}

#[no_mangle]
pub extern "C" fn add_redirect(
    builder_config_ptr: *mut BuilderConfig,
    from: *const c_char,
    to: *const c_char,
) {
    let builder = unsafe { &mut *builder_config_ptr };

    let from: Uri = get_string_from_cstr_ptr(from).try_into().unwrap();
    let to: Uri = get_string_from_cstr_ptr(to).try_into().unwrap();

    builder.add_redirect(from, to);
}

#[no_mangle]
pub extern "C" fn remove_redirect(builder_config_ptr: *mut BuilderConfig, from: *const c_char) {
    let builder = unsafe { &mut *builder_config_ptr };
    let from: Uri = get_string_from_cstr_ptr(from).try_into().unwrap();

    builder.remove_redirect(from);
}

#[no_mangle]
pub extern "C" fn add_wrapper_resolver(
    builder_config_ptr: *mut BuilderConfig,
    resolver: SafeUriResolverLikeVariant,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    builder.add_resolver(resolver.into());
}

#[no_mangle]
pub extern "C" fn add_redirect_resolver(
    builder_config_ptr: *mut BuilderConfig,
    resolver: SafeUriResolverLikeVariant,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    builder.add_resolver(resolver.into());
}

#[no_mangle]
pub extern "C" fn add_package_resolver(
    builder_config_ptr: *mut BuilderConfig,
    resolver: SafeUriResolverLikeVariant,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    builder.add_resolver(resolver.into());
}

#[no_mangle]
pub extern "C" fn add_resolver(
    builder_config_ptr: *mut BuilderConfig,
    resolver: SafeUriResolverLikeVariant,
) {
    let builder = unsafe { &mut *builder_config_ptr };
    builder.add_resolver(resolver.into());
}
