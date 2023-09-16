use polywrap_core::{
    client::{CoreClient, CoreClientConfig},
    error::Error,
    interface_implementation::InterfaceImplementations,
    invoker::Invoker,
    resolution::uri_resolution_context::UriResolutionContext,
    resolution::{
        helpers::get_env_from_resolution_path,
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionStep},
        uri_resolver::UriResolver,
    },
    uri::Uri,
    uri_resolver_handler::UriResolverHandler,
    wrap_invoker::WrapInvoker,
    wrap_loader::WrapLoader,
    wrapper::Wrapper,
};
use polywrap_msgpack_serde::from_slice;
use polywrap_plugin::InvokerContext;
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::subinvoker::Subinvoker;

#[derive(Clone, Debug)]
pub struct Client {
    pub resolver: Arc<dyn UriResolver>,
    pub envs: Option<HashMap<Uri, Vec<u8>>>,
    pub interfaces: Option<InterfaceImplementations>,
}

impl Client {
    pub fn new(config: CoreClientConfig) -> Self {
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
        context: Option<InvokerContext>,
    ) -> Result<T, Error> {
        let result = self.invoke_raw(uri, method, args, context)?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn invoke_wrapper<TResult: DeserializeOwned, TWrapper: Wrapper>(
        &self,
        wrapper: &TWrapper,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        context: Option<InvokerContext>,
    ) -> Result<TResult, Error> {
        let result =
            self.invoke_wrapper_raw(wrapper, uri, method, args, context)?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}

impl Invoker for Client {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        context: Option<InvokerContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut default_resolution_context = UriResolutionContext::new();
        let context = context.unwrap_or(InvokerContext::default(&mut default_resolution_context));

        let mut loaded_wrapper_context = context.resolution_context.create_sub_context();

        let load_result = self
            .clone()
            .load_wrapper(uri, Some(&mut loaded_wrapper_context));

        if load_result.is_err() {
            let error = load_result.err().unwrap();

            context.resolution_context
                .track_step(UriResolutionStep {
                    source_uri: uri.clone(),
                    result: Err(error.clone()),
                    description: Some(format!("Client.loadWrapper({uri})")),
                    sub_history: Some(loaded_wrapper_context.get_history().clone()),
                });

            return Err(Error::LoadWrapperError(uri.to_string(), error.to_string()));
        }

        let resolution_path = loaded_wrapper_context.get_resolution_path();
        let resolution_path = if !resolution_path.is_empty() {
            resolution_path
        } else {
            vec![uri.clone()]
        };

        let resolved_uri = resolution_path.last().unwrap();

        let wrapper = load_result.unwrap();

        context.resolution_context
            .track_step(UriResolutionStep {
                source_uri: uri.clone(),
                result: Ok(UriPackageOrWrapper::Wrapper(
                    resolved_uri.clone(),
                    wrapper.clone(),
                )),
                description: Some("Client.loadWrapper".to_string()),
                sub_history: Some(loaded_wrapper_context.get_history().clone()),
            });

        let env = if context.env.is_some() {
            context.env.map(|e| e.to_vec())
        } else {
            get_env_from_resolution_path(&resolution_path, self)
        };

        let result = self.invoke_wrapper_raw(
            &*wrapper,
            uri,
            method,
            args,
            Some(InvokerContext {
                resolution_context: context.resolution_context,
                env: env.as_deref(),
                caller_context: context.caller_context,
                overriden_own_context: context.overriden_own_context,
            })
        );

        result
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

    fn get_env_by_uri(&self, uri: &Uri) -> Option<Vec<u8>> {
        if let Some(envs) = &self.envs {
            return envs.get(uri).cloned();
        }

        None
    }
}

impl WrapLoader for Client {
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<dyn Wrapper>, Error> {
        let mut default_resolution_context = UriResolutionContext::new();
        let resolution_context = match resolution_context {
            None => &mut default_resolution_context,
            Some(ctx) => ctx,
        };

        let uri_package_or_wrapper = self
            .try_resolve_uri(uri, Some(resolution_context))
            .map_err(|e| Error::ResolutionError(e.to_string()))?;

        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri) => Err(Error::UriNotFoundError(uri.to_string())),
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

impl WrapInvoker for Client {
    fn invoke_wrapper_raw(
        &self,
        wrapper: &dyn Wrapper,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        context: Option<InvokerContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut default_resolution_context = UriResolutionContext::new();
        let context = context.unwrap_or(InvokerContext::default(&mut default_resolution_context));
        let subinvocation_context = context.resolution_context.create_sub_context();
        let subinvocation_context = Arc::new(Mutex::new(subinvocation_context));

        let subinvoker = Arc::new(Subinvoker::new(
            Arc::new(self.clone()),
            context.resolution_context.clone(),
            context.caller_context.cloned(),
            context.overriden_own_context.cloned(),
            subinvocation_context.clone(),
        ));

        let invoke_result = wrapper
            .invoke(method, args, context.env, subinvoker)
            .map_err(|e| Error::InvokeError(uri.to_string(), method.to_string(), e.to_string()));

        let subinvocation_context = subinvocation_context.lock().unwrap();

        context.resolution_context.track_step(UriResolutionStep {
            source_uri: uri.clone(),
            result: if invoke_result.is_ok() {
                Ok(UriPackageOrWrapper::Uri(uri.clone()))
            } else {
                Err(invoke_result.clone().unwrap_err())
            },
            description: Some("Client.invokeWrapper".to_string()),
            sub_history: Some(subinvocation_context.get_history().clone()),
        });

        invoke_result
    }
}

impl UriResolverHandler for Client {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_resolver = self.resolver.clone();
        let mut default_resolution_context = UriResolutionContext::new();
        let resolution_context = match resolution_context {
            Some(r) => r,
            None => &mut default_resolution_context,
        };

        uri_resolver.try_resolve_uri(uri, Arc::new(self.clone()), resolution_context)
    }
}

impl CoreClient for Client {}

#[cfg(test)]
mod client_tests {
    use polywrap_core::{
        client::CoreClientConfig, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri,
        uri_resolver_handler::UriResolverHandler, wrap_loader::WrapLoader,
    };
    use polywrap_msgpack_serde::from_slice;
    use polywrap_tests_utils::mocks::{get_mock_resolver, MockWrapper};
    use std::sync::Arc;

    use super::Client;

    #[test]
    fn invoke() {
        let client = Client::new(CoreClientConfig {
            resolver: get_mock_resolver(),
            envs: None,
            interfaces: None,
        });

        let result = client
            .invoke::<bool>(&"wrap/mock".try_into().unwrap(), "foo", None, None, None)
            .unwrap();

        assert!(result);
    }

    #[test]
    fn invoke_wrapper() {
        let client = Client::new(CoreClientConfig {
            resolver: get_mock_resolver(),
            envs: None,
            interfaces: None,
        });

        let wrapper = MockWrapper {};

        let result = client
            .invoke_wrapper::<bool, MockWrapper>(
                &wrapper,
                &"wrap/mock".try_into().unwrap(),
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
        let client = Client::new(CoreClientConfig {
            resolver: get_mock_resolver(),
            envs: None,
            interfaces: None,
        });

        let wrapper = client
            .load_wrapper(&"wrap/mock".try_into().unwrap(), None)
            .unwrap();

        let result = wrapper.invoke("foo", None, None, Arc::new(client));
        let r = result.unwrap();
        assert!(from_slice::<bool>(&r).unwrap());
    }

    #[test]
    fn try_resolve_uri() {
        let client = Client::new(CoreClientConfig {
            resolver: get_mock_resolver(),
            envs: None,
            interfaces: None,
        });
        let uri: Uri = "wrap/mock".try_into().unwrap();

        let uri_package_or_wrapper = client.try_resolve_uri(&uri, None).unwrap();

        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(_) => panic!("Found Uri, should've found MockWrapper"),
            UriPackageOrWrapper::Wrapper(_, wrapper) => {
                let result = wrapper.invoke("foo", None, None, Arc::new(client));
                let r = result.unwrap();
                assert!(from_slice::<bool>(&r).unwrap());
            }
            UriPackageOrWrapper::Package(_, _) => panic!("Found Uri, should've found MockWrapper"),
        }
    }
}
