use std::sync::Arc;

use polywrap_core::{
    client::{Client, ClientConfig},
    env::{Env, Envs},
    error::Error,
    interface_implementation::InterfaceImplementations,
    invoker::Invoker,
    resolvers::uri_resolution_context::UriResolutionContext,
    resolvers::{
        uri_resolution_context::UriPackageOrWrapper,
        uri_resolver::{UriResolver, UriResolverHandler},
    },
    uri::Uri,
    wrapper::Wrapper,
};
use polywrap_msgpack::decode;
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct PolywrapClient {
    pub resolver: Arc<dyn UriResolver>,
    pub envs: Option<Envs>,
    pub interfaces: Option<InterfaceImplementations>,
}

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        let resolver = config.resolver;
        let envs = config.envs;
        let interfaces = config.interfaces;
        Self {
            resolver,
            envs,
            interfaces,
        }
    }

    pub fn invoke<T: DeserializeOwned>(
      &self,
      uri: &Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&Env>,
      resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<T, Error> {
      let result = self.invoke_raw(uri, method, args, env, resolution_context)?;

      decode(result.as_slice())
          .map_err(|e| Error::InvokeError(format!("Failed to decode result: {e}")))
  }

  pub fn invoke_wrapper<TResult: DeserializeOwned, TWrapper: Wrapper>(
      &self,
      wrapper: &TWrapper,
      uri: &Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&Env>,
      resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<TResult, Error> {
      let result = wrapper
          .invoke(Arc::new(self.clone()), uri, method, args, env, resolution_context)
          .map_err(|e| Error::InvokeError(e.to_string()))?;

      decode(result.as_slice())
          .map_err(|e| Error::InvokeError(format!("Failed to decode result: {e}")))
  }
}

impl Invoker for PolywrapClient {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let mut resolution_context = match resolution_context {
            None => &mut empty_res_context,
            Some(ctx) => ctx,
        };

        let wrapper = self
            .clone()
            .load_wrapper(uri, Some(&mut resolution_context))
            .map_err(|e| Error::LoadWrapperError(e.to_string()))?;

        let self_clone = self.clone();
        let mut env = env;
        if env.is_none() {
            env = self_clone.get_env_by_uri(uri);
        }

        let invoke_result = self
            .invoke_wrapper_raw(wrapper, uri, method, args, env, Some(resolution_context))
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(invoke_result)
    }

    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
        polywrap_core::resolvers::helpers::get_implementations(uri, self.get_interfaces(), self)
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        if let Some(interfaces) = self.interfaces.clone() {
            return Some(interfaces);
        }

        None
    }
}

impl Client for PolywrapClient {
    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env> {
        if let Some(envs) = &self.envs {
            return envs.get(&uri.to_string());
        }

        None
    }

    fn load_wrapper(
      &self,
      uri: &Uri,
      resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<Arc<dyn Wrapper>, Error> {
      let mut empty_res_context = UriResolutionContext::new();
      let mut resolution_ctx = match resolution_context {
          Some(ctx) => ctx,
          None => &mut empty_res_context,
      };
  
      let uri_package_or_wrapper = self
          .try_resolve_uri(uri, Some(&mut resolution_ctx))
          .map_err(|e| Error::ResolutionError(e.to_string()))?;
  
      match uri_package_or_wrapper {
          UriPackageOrWrapper::Uri(uri) => Err(Error::InvokeError(format!(
              "Failed to resolve wrapper: {uri}"
          ))),
          UriPackageOrWrapper::Wrapper(_, wrapper) => Ok(wrapper),
          UriPackageOrWrapper::Package(_, package) => {
              let wrapper = package
                  .create_wrapper()
                  .map_err(|e| Error::WrapperCreateError(e.to_string()))?;
              Ok(wrapper)
          }
      }
    }

    fn invoke_wrapper_raw(
      &self,
      wrapper: Arc<dyn Wrapper>,
      uri: &Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&Env>,
      resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        wrapper
            .invoke(Arc::new(self.clone()), uri, method, args, env, resolution_context)
            .map_err(|e| Error::InvokeError(e.to_string()))
    }
}

impl UriResolverHandler for PolywrapClient {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_resolver = self.resolver.clone();
        let mut uri_resolver_context = UriResolutionContext::new();

        let resolution_context = match resolution_context {
            Some(ctx) => ctx,
            None => &mut uri_resolver_context,
        };

        uri_resolver.try_resolve_uri(uri, Arc::new(self.clone()), resolution_context)
    }
}

#[cfg(test)]
mod client_tests {
    use crate::client::Env;
    use polywrap_core::{
        client::{ClientConfig, Client},
        error::Error,
        invoker::Invoker,
        resolvers::{
            uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
            uri_resolver::{UriResolver, UriResolverHandler},
        },
        uri::Uri,
        wrapper::{GetFileOptions, Wrapper},
    };
    use std::sync::Arc;

    use super::PolywrapClient;

    #[derive(Debug)]
    struct MockWrapper {
        id: u32,
    }

    impl MockWrapper {
        pub fn new(id: u32) -> Self {
            MockWrapper { id }
        }
    }

    impl Wrapper for MockWrapper {
        fn invoke(
            &self,
            _: Arc<dyn Invoker>,
            _: &Uri,
            method: &str,
            _: Option<&[u8]>,
            _: Option<&Env>,
            _: Option<&mut UriResolutionContext>,
        ) -> Result<Vec<u8>, Error> {
            // In Msgpack: True = [195] and False = [194]

            if method == "foo" {
                Ok(vec![195])
            } else {
                Ok(vec![194])
            }
        }

        fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
            unimplemented!()
        }
    }

    #[derive(Debug)]
    struct MockResolver {}

    impl UriResolver for MockResolver {
        fn try_resolve_uri(
            &self,
            uri: &Uri,
            _: Arc<dyn Invoker>,
            _: &mut UriResolutionContext,
        ) -> Result<UriPackageOrWrapper, Error> {
            if uri.to_string() == *"wrap://ens/mock.eth" {
                Ok(UriPackageOrWrapper::Wrapper(
                    "wrap://ens/mock.eth".try_into().unwrap(),
                    Arc::new(MockWrapper::new(54)),
                ))
            } else {
                Err(Error::InvokeError("Not Found".to_string()))
            }
        }
    }

    #[test]
    fn invoke() {
        let client = PolywrapClient::new(ClientConfig {
            resolver: Arc::new(MockResolver {}),
            envs: None,
            interfaces: None,
        });

        let result = client
            .invoke::<bool>(
                &"wrap://ens/mock.eth".try_into().unwrap(),
                "foo",
                None,
                None,
                None,
            )
            .unwrap();

        assert!(result);
    }

    #[test]
    fn invoke_wrapper() {
        let client = PolywrapClient::new(ClientConfig {
            resolver: Arc::new(MockResolver {}),
            envs: None,
            interfaces: None,
        });

        let wrapper = MockWrapper::new(100);

        let result = client
            .invoke_wrapper::<bool, MockWrapper>(
                &wrapper,
                &"wrap://ens/mock.eth".try_into().unwrap(),
                "foo",
                None,
                None,
                None,
            )
            .unwrap();

        assert!(result);
    }

    #[test]
    fn load_wrapper() {
        let client = PolywrapClient::new(ClientConfig {
            resolver: Arc::new(MockResolver {}),
            envs: None,
            interfaces: None,
        });

        let wrapper = client
            .load_wrapper(&"wrap://ens/mock.eth".try_into().unwrap(), None)
            .unwrap();
        let wrapper = &*(wrapper) as &dyn std::any::Any;
        let wrapper = wrapper.downcast_ref::<MockWrapper>().unwrap();

        assert_eq!(wrapper.id, 54);
    }

    #[test]
    fn try_resolve_uri() {
        let client = PolywrapClient::new(ClientConfig {
            resolver: Arc::new(MockResolver {}),
            envs: None,
            interfaces: None,
        });
        let uri: Uri = "wrap://ens/mock.eth".try_into().unwrap();

        let uri_package_or_wrapper = client.try_resolve_uri(&uri, None).unwrap();
        
        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(_) => panic!("Found Uri, should've found MockWrapper"),
            UriPackageOrWrapper::Wrapper(_, wrapper) => {
              let wrapper = &*(wrapper) as &dyn std::any::Any;
              let wrapper = wrapper.downcast_ref::<MockWrapper>().unwrap();

              assert_eq!(wrapper.id, 54);
            },
            UriPackageOrWrapper::Package(_, _) => panic!("Found Uri, should've found MockWrapper"),
        }
    }
}