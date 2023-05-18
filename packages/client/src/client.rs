use std::sync::{Arc, Mutex};

use polywrap_core::{
    client::{Client, ClientConfig},
    env::{Env, Envs},
    error::Error,
    interface_implementation::InterfaceImplementations,
    invoker::Invoker,
    resolution::uri_resolution_context::UriResolutionContext,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionStep},
        uri_resolver::UriResolver, helpers::get_env_from_resolution_path,
    },
    uri::Uri,
    wrapper::Wrapper, wrap_loader::WrapLoader, wrap_invoker::WrapInvoker, uri_resolver_handler::UriResolverHandler,
};
use polywrap_msgpack::decode;
use serde::de::DeserializeOwned;

use crate::{subinvoker::Subinvoker, build_abort_handler::build_abort_handler};

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
          .map_err(|e| Error::MsgpackError(format!("Failed to decode result: {e}")))
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
        let result = self.invoke_wrapper_raw(
            wrapper,
            uri,
            method,
            args,
            env,
            resolution_context,
        )?;

      decode(result.as_slice())
          .map_err(|e| Error::MsgpackError(format!("Failed to decode result: {e}")))
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

        let mut loaded_wrapper_context = resolution_context.create_sub_context();

        let load_result = self
            .clone()
            .load_wrapper(uri, Some(&mut loaded_wrapper_context));

        if load_result.is_err() {
            let error = load_result.err().unwrap();

            resolution_context.track_step(UriResolutionStep {
                source_uri: uri.clone(),
                result: Err(error.clone()),
                description: Some(format!("Client.loadWrapper({uri})")),
                sub_history: Some(loaded_wrapper_context.get_history().clone())
            });

            return Err(Error::LoadWrapperError(error.to_string()));
        }

        let resolution_path = loaded_wrapper_context.get_resolution_path();
        let resolution_path = if resolution_path.len() > 0 {
            resolution_path
        } else {
            vec![uri.clone()]
        };

        let resolved_uri = resolution_path.last().unwrap();

        let wrapper = load_result.unwrap();

        resolution_context.track_step(UriResolutionStep {
            source_uri: uri.clone(),
            result: Ok(UriPackageOrWrapper::Wrapper(resolved_uri.clone(), wrapper.clone())),
            description: Some("Client.loadWrapper".to_string()),
            sub_history: Some(loaded_wrapper_context.get_history().clone())
        });

        let env = if env.is_some() {
            env
        } else {
            get_env_from_resolution_path(&resolution_path, self)
        };

        self
            .invoke_wrapper_raw(&*wrapper, uri, method, args, env, Some(&mut resolution_context))
    }

    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
        polywrap_core::resolution::helpers::get_implementations(uri, self.get_interfaces(), self)
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        if let Some(interfaces) = self.interfaces.clone() {
            return Some(interfaces);
        }

        None
    }

    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env> {
        if let Some(envs) = &self.envs {
            return envs.get(&uri.to_string());
        }

        None
    }
}

impl WrapLoader for PolywrapClient {
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<dyn Wrapper>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let mut resolution_context = match resolution_context {
            None => &mut empty_res_context,
            Some(ctx) => ctx,
        };
    
        let uri_package_or_wrapper = self
            .try_resolve_uri(uri, Some(&mut resolution_context))
            .map_err(|e| Error::ResolutionError(e.to_string()))?;
    
        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri) => Err(Error::ResolutionError(format!(
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
}

impl WrapInvoker for PolywrapClient {
    fn invoke_wrapper_raw(
        &self,
        wrapper: &dyn Wrapper,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let resolution_context = match resolution_context {
            None => &mut empty_res_context,
            Some(ctx) => ctx,
        };

        let subinvocation_context = resolution_context.create_sub_context();
        let subinvocation_context = Arc::new(Mutex::new(subinvocation_context));

        let subinvoker = Arc::new(Subinvoker::new(
            Arc::new(self.clone()),
            subinvocation_context.clone(),
        ));

        let abort_handler = build_abort_handler(None, uri.clone(), method.to_string());

        let invoke_result = wrapper
            .invoke(method, args, env, subinvoker.clone(), Some(abort_handler))
            .map_err(|e| Error::InvokeError(uri.to_string(), method.to_string(), e.to_string()));

        let subinvocation_context = subinvocation_context.lock().unwrap();

        resolution_context.track_step(UriResolutionStep {
            source_uri: uri.clone(),
            result: if invoke_result.is_ok() {
                Ok(UriPackageOrWrapper::Uri(uri.clone()))
            } else {
                Err(invoke_result.clone().unwrap_err())
            },
            description: Some("Client.invokeWrapper".to_string()),
            sub_history: Some(subinvocation_context.get_history().clone())
        });

        invoke_result
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

impl Client for PolywrapClient {
}

#[cfg(test)]
mod client_tests {
    use crate::client::Env;
    use polywrap_core::{
        client::ClientConfig,
        error::Error,
        invoker::Invoker,
        resolution::{
            uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
            uri_resolver::{UriResolver, UriResolverHandler},
        },
        uri::Uri,
        wrapper::{GetFileOptions, Wrapper}, wrap_loader::WrapLoader, uri_resolver_handler::UriResolverHandler,
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
            method: &str,
            _: Option<&[u8]>,
            _: Option<&Env>,
            _: Arc<dyn Invoker>,
            _: Option<Box<dyn Fn(String) + Send + Sync>>,
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
                Err(Error::ResolutionError("Not Found".to_string()))
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