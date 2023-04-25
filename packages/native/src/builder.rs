use polywrap_client::{
    builder::types::{BuilderConfig, ClientBuilder},
    core::{package::WrapPackage, resolvers::uri_resolver_like::UriResolverLike},
};
use std::sync::{Arc, Mutex};

use crate::{
    plugin_wrapper::FFIPluginWrapper,
    resolvers::{
        _static::FFIStaticUriResolver,
        extendable::FFIExtendableUriResolver,
        ffi_resolver::{FFIUriResolver, FFIUriResolverWrapper},
        recursive::FFIRecursiveUriResolver,
    },
    wasm_wrapper::FFIWasmWrapper,
};

pub struct FFIBuilderConfig {
    pub inner_builder: Mutex<BuilderConfig>,
}

impl FFIBuilderConfig {
    pub fn new() -> FFIBuilderConfig {
        FFIBuilderConfig {
            inner_builder: Mutex::new(BuilderConfig::new(None)),
        }
    }

    pub fn add_env(&self, uri: &str, env: &str) {
        self.inner_builder.lock().unwrap().add_env(
            uri.to_string().try_into().unwrap(),
            serde_json::from_str(env).unwrap(),
        );
    }

    pub fn remove_env(&self, uri: &str) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_env(uri.to_string().try_into().unwrap());
    }

    pub fn set_env(&self, uri: &str, env: &str) {
        self.inner_builder.lock().unwrap().set_env(
            uri.to_string().try_into().unwrap(),
            serde_json::from_str(env).unwrap(),
        );
    }

    pub fn add_interface_implementation(&self, interface_uri: &str, implementation_uri: &str) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_interface_implementation(
                interface_uri.try_into().unwrap(),
                implementation_uri.try_into().unwrap(),
            );
    }

    pub fn remove_interface_implementation(&self, interface_uri: &str, implementation_uri: &str) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_interface_implementation(
                interface_uri.try_into().unwrap(),
                implementation_uri.try_into().unwrap(),
            );
    }

    pub fn add_wasm_wrapper(&self, uri: &str, wrapper: Arc<FFIWasmWrapper>) {
        self.inner_builder.lock().unwrap().add_wrapper(
            uri.to_string().try_into().unwrap(),
            wrapper.inner_wasm_wrapper.clone(),
        );
    }

    pub fn add_plugin_wrapper(&self, uri: &str, wrapper: Arc<FFIPluginWrapper>) {
        self.inner_builder.lock().unwrap().add_wrapper(
            uri.to_string().try_into().unwrap(),
            wrapper.inner_plugin.clone(),
        );
    }

    pub fn remove_wrapper(&self, uri: &str) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_wrapper(uri.try_into().unwrap());
    }

    pub fn add_package(&self, uri: &str, package: Box<dyn WrapPackage>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_package(uri.try_into().unwrap(), Arc::new(Mutex::new(package)));
    }

    pub fn remove_package(&self, uri: &str) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_package(uri.try_into().unwrap());
    }

    pub fn add_redirect(&self, from: &str, to: &str) {
        self.inner_builder.lock().unwrap().add_redirect(
            from.to_string().try_into().unwrap(),
            to.to_string().try_into().unwrap(),
        );
    }

    pub fn remove_redirect(&self, from: &str) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_redirect(from.to_string().try_into().unwrap());
    }

    pub fn add_resolver(&self, resolver: Box<dyn FFIUriResolver>) {
        let resolver: FFIUriResolverWrapper = resolver.into();
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn add_static_resolver(&self, resolver: FFIStaticUriResolver) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn add_extendable_resolver(&self, resolver: FFIExtendableUriResolver) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn add_recursive_resolver(&self, resolver: FFIRecursiveUriResolver) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }
}
