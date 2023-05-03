use std::sync::Arc;

use polywrap_core::{
    client::{Client, ClientConfig},
    env::{Env, Envs},
    error::Error,
    interface_implementation::InterfaceImplementations,
    invoke::Invoker,
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

struct InnerPolywrapClient {
    pub resolver: Arc<dyn UriResolver>,
    pub envs: Option<Envs>,
    pub interfaces: Option<InterfaceImplementations>,
}

impl InnerPolywrapClient {
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
}

impl Invoker for InnerPolywrapClient {
    fn invoke_wrapper_raw(
        self: Arc<Self>,
        wrapper: Arc<dyn Wrapper>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper
            .invoke(self.clone(), uri, method, args, env, resolution_context)
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(result)
    }

    fn invoke_raw(
        self: Arc<Self>,
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

impl Client for InnerPolywrapClient {
    fn load_wrapper(
        self: Arc<Self>,
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

    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env> {
        if let Some(envs) = &self.envs {
            return envs.get(&uri.uri);
        }

        None
    }
}

impl UriResolverHandler for InnerPolywrapClient {
    fn try_resolve_uri(
        self: Arc<Self>,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_resolver = self.resolver.clone();
        let mut uri_resolver_context = UriResolutionContext::new();

        let resolution_context = match resolution_context {
            Some(ctx) => ctx,
            None => &mut uri_resolver_context,
        };

        uri_resolver.try_resolve_uri(uri, self.clone(), resolution_context)
    }
}

pub struct PolywrapClient(Arc<InnerPolywrapClient>);

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        let resolver = config.resolver;
        let envs = config.envs;
        let interfaces = config.interfaces;
        let inner_client = InnerPolywrapClient::new(ClientConfig {
            resolver,
            envs,
            interfaces,
        });

        PolywrapClient(Arc::new(inner_client))
    }

    pub fn invoke<T: DeserializeOwned>(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self
            .0
            .clone()
            .invoke_raw(uri, method, args, env, resolution_context)?;

        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {e}")))
    }

    pub fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<dyn Wrapper>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        wrapper
            .invoke(self.0.clone(), uri, method, args, env, resolution_context)
            .map_err(|e| Error::InvokeError(e.to_string()))
    }

    pub fn invoke_wrapper<TWrapper: Wrapper, TResult: DeserializeOwned>(
        &self,
        wrapper: &TWrapper,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<TResult, Error> {
        let result = wrapper
            .invoke(self.0.clone(), uri, method, args, env, resolution_context)
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {e}")))
    }

    pub fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error> {
        self.0.clone().try_resolve_uri(uri, resolution_context)
    }

    pub fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<dyn Wrapper>, Error> {
        self.0.clone().load_wrapper(uri, resolution_context)
    }

    pub fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        self.0
            .clone()
            .invoke_raw(uri, method, args, env, resolution_context)
    }

    pub fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
        self.0.get_implementations(uri)
    }

    pub fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        self.0.get_interfaces()
    }

    pub fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env> {
        self.0.get_env_by_uri(uri)
    }
}

#[cfg(test)]
mod client_tests {
    use crate::client::Env;
    use polywrap_core::{
        client::{Client, ClientConfig},
        error::Error,
        invoke::Invoker,
        resolvers::{
            uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
            uri_resolver::UriResolver,
        },
        uri::Uri,
        wrapper::{GetFileOptions, Wrapper},
    };
    use std::sync::Arc;

    use super::PolywrapClient;

    #[derive(Debug)]
    struct MockWrapper {}

    impl Wrapper for MockWrapper {
        fn invoke(
            &self,
            invoker: Arc<dyn Invoker>,
            uri: &Uri,
            method: &str,
            args: Option<&[u8]>,
            env: Option<&Env>,
            resolution_context: Option<&mut UriResolutionContext>,
        ) -> Result<Vec<u8>, Error> {
            // In Msgpack: True = [195]
            Ok(vec![195])
        }

        fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error> {
            unimplemented!()
        }
    }

    #[derive(Debug)]
    struct MockResolver {}

    impl UriResolver for MockResolver {
        fn try_resolve_uri(
            &self,
            uri: &Uri,
            client: Arc<dyn Client>,
            resolution_context: &mut UriResolutionContext,
        ) -> Result<UriPackageOrWrapper, Error> {
            Ok(UriPackageOrWrapper::Wrapper(
                "wrap://ens/mock.eth".try_into().unwrap(),
                Arc::new(MockWrapper {}),
            ))
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
}
