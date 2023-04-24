use polywrap_client::{
    builder::types::{BuilderConfig as InnerBuilderConfig, ClientBuilder},
    core::{
        package::WrapPackage,
        resolvers::{uri_resolver::UriResolver, uri_resolver_like::UriResolverLike}
    },
};
use std::sync::{Arc, Mutex};

use crate::{plugin_wrapper::FFIPluginWrapper, wasm_wrapper::FFIWasmWrapper};

pub struct BuilderConfig {
    pub inner_builder: Mutex<InnerBuilderConfig>,
}

impl BuilderConfig {
    pub fn new() -> BuilderConfig {
        BuilderConfig {
            inner_builder: Mutex::new(InnerBuilderConfig::new(None)),
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

    pub fn add_resolver(&self, resolver: Box<dyn UriResolver>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }
}
