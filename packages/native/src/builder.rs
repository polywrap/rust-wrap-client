use polywrap_client::{
    builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler},
    client::PolywrapClient,
};
use std::sync::{Arc, Mutex};

use crate::{
    resolvers::{
        ffi_resolver::{FFIUriResolver, ExtUriResolver},
    },
    client::FFIClient, uri::FFIUri, wrapper::{FFIWrapper, ExtWrapper}, package::{FFIWrapPackage, ExtWrapPackage},
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

    pub fn add_env(&self, uri: Arc<FFIUri>, env: Vec<u8>) {
        self.inner_builder.lock().unwrap().add_env(
            uri.0.clone(),
            env,
        );
    }

    pub fn remove_env(&self, uri: Arc<FFIUri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_env(&uri.0);
    }

    pub fn set_env(&self, uri: Arc<FFIUri>, env: Vec<u8>) {
        self.inner_builder.lock().unwrap().set_env(
            uri.0.clone(),
            env,
        );
    }

    pub fn add_interface_implementation(&self, interface_uri: Arc<FFIUri>, implementation_uri: Arc<FFIUri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_interface_implementation(
                interface_uri.0.clone(),
                implementation_uri.0.clone(),
            );
    }

    pub fn remove_interface_implementation(&self, interface_uri: Arc<FFIUri>, implementation_uri: Arc<FFIUri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_interface_implementation(
                &interface_uri.0,
                &implementation_uri.0
            );
    }

    pub fn add_wrapper(&self, uri: Arc<FFIUri>, wrapper: Box<dyn FFIWrapper>) {
        self.inner_builder.lock().unwrap().add_wrapper(
            uri.0.clone(),
            Arc::new(ExtWrapper(wrapper)),
        );
    }

    pub fn remove_wrapper(&self, uri: Arc<FFIUri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_wrapper(&uri.0);
    }

    pub fn add_package(&self, uri: Arc<FFIUri>, package: Box<dyn FFIWrapPackage>) {
      self.inner_builder.lock().unwrap().add_package(
          uri.0.clone(),
          Arc::new(ExtWrapPackage(package)),
      );
    }

    pub fn remove_package(&self, uri: Arc<FFIUri>) {
      self.inner_builder
          .lock()
          .unwrap()
          .remove_package(&uri.0);
  }

    pub fn add_redirect(&self, from: Arc<FFIUri>, to: Arc<FFIUri>) {
        self.inner_builder.lock().unwrap().add_redirect(
            from.0.clone(),
            to.0.clone(),
        );
    }

    pub fn remove_redirect(&self, from: Arc<FFIUri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_redirect(&from.0);
    }

    pub fn add_resolver(&self, resolver: Box<dyn FFIUriResolver>) {
        let resolver = ExtUriResolver(resolver);
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(Arc::new(resolver));
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

    use polywrap_client::msgpack::msgpack;

    use crate::uri::FFIUri;

    use super::FFIBuilderConfig;

    #[test]
    fn it_adds_env() {
        let builder = FFIBuilderConfig::new();
        let uri = Arc::new(FFIUri::from_string("wrap://ens/some.eth"));
        let env = msgpack!({
          "foo": "bar"
        });

        builder.add_env(uri.clone(), env.clone());

        let ffi_client = builder.build();
        let found_env = ffi_client.get_env_by_uri(uri);

        assert_eq!(found_env.unwrap(), env.as_slice());
    }
}