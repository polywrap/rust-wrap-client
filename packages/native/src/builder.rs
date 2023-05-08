use polywrap_client::{
    builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler},
    core::{resolvers::uri_resolver_like::UriResolverLike, uri::Uri}, client::PolywrapClient,
};
use polywrap_plugin::{module::PluginModule, wrapper::PluginWrapper};
use std::sync::{Arc, Mutex};

use crate::{
    resolvers::{
        _static::FFIStaticUriResolver,
        extendable::FFIExtendableUriResolver,
        ffi_resolver::{FFIUriResolver, FFIUriResolverWrapper},
        recursive::FFIRecursiveUriResolver,
    },
    wasm_wrapper::FFIWasmWrapper, client::FFIClient, plugin_wrapper::FFIPluginModule,
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

    pub fn add_env(&self, uri: Arc<Uri>, env: &str) {
        self.inner_builder.lock().unwrap().add_env(
            uri.as_ref().clone(),
            serde_json::from_str(env).unwrap(),
        );
    }

    pub fn remove_env(&self, uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_env(uri.as_ref());
    }

    pub fn set_env(&self, uri: Arc<Uri>, env: &str) {
        self.inner_builder.lock().unwrap().set_env(
            uri.as_ref().clone(),
            serde_json::from_str(env).unwrap(),
        );
    }

    pub fn add_interface_implementation(&self, interface_uri: Arc<Uri>, implementation_uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_interface_implementation(
                interface_uri.as_ref().clone(),
                implementation_uri.as_ref().clone(),
            );
    }

    pub fn remove_interface_implementation(&self, interface_uri: Arc<Uri>, implementation_uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_interface_implementation(
                interface_uri.as_ref(),
                implementation_uri.as_ref()
            );
    }

    pub fn add_wasm_wrapper(&self, uri: Arc<Uri>, wrapper: Arc<FFIWasmWrapper>) {
        self.inner_builder.lock().unwrap().add_wrapper(
            uri.as_ref().clone(),
            wrapper.inner_wasm_wrapper.clone(),
        );
    }

    pub fn add_plugin_wrapper(&self, uri: Arc<Uri>, plugin_module: Box<dyn FFIPluginModule>) {
        let plugin_instance = Box::new(plugin_module) as Box<dyn PluginModule>;
        let plugin = PluginWrapper::new(Arc::new(Mutex::new(plugin_instance)));
        self.inner_builder.lock().unwrap().add_wrapper(
            uri.as_ref().clone(),
            Arc::new(plugin),
        );
    }

    pub fn remove_wrapper(&self, uri: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_wrapper(uri.as_ref());
    }

    pub fn add_redirect(&self, from: Arc<Uri>, to: Arc<Uri>) {
        self.inner_builder.lock().unwrap().add_redirect(
            from.as_ref().clone(),
            to.as_ref().clone(),
        );
    }

    pub fn remove_redirect(&self, from: Arc<Uri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_redirect(from.as_ref());
    }

    pub fn add_resolver(&self, resolver: Box<dyn FFIUriResolver>) {
        let resolver: FFIUriResolverWrapper = resolver.into();
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
    }

    pub fn add_static_resolver(&self, resolver: Arc<FFIStaticUriResolver>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(resolver));
    }

    pub fn add_extendable_resolver(&self, resolver: Arc<FFIExtendableUriResolver>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(resolver));
    }

    pub fn add_recursive_resolver(&self, resolver: Arc<FFIRecursiveUriResolver>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(UriResolverLike::Resolver(resolver));
    }

    pub fn build(&self) -> Arc<FFIClient> {
      let config = self.inner_builder.lock().unwrap().clone().build();
      let client = Arc::new(PolywrapClient::new(config));
      Arc::new(FFIClient::new(client))
    }
}

#[cfg(test)]
mod builder_tests {
    use std::sync::Arc;

    use polywrap_client::core::uri::Uri;
    use serde_json::json;

    use super::FFIBuilderConfig;

    #[test]
    fn it_adds_env() {
        let builder = FFIBuilderConfig::new();
        let uri = Arc::new(Uri::new("wrap://ens/some.eth"));
        let env = json!({
          "foo": "bar"
        }).to_string();

        builder.add_env(uri.clone(), &env);

        let ffi_client = builder.build();
        let found_env = ffi_client.get_env_by_uri(uri);

        assert_eq!(found_env.unwrap(), env);
    }
}