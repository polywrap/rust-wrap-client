use std::sync::{Arc, Mutex};

use polywrap_core::{
    client::{Client, ClientConfig},
    env::{Env, Envs},
    error::Error,
    interface_implementation::InterfaceImplementations,
    invoke::Invoker,
    loader::Loader,
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

#[derive(Clone)]
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

    pub fn invoke_wrapper<T: DeserializeOwned>(
        &self,
        wrapper: Arc<Mutex<Box<dyn Wrapper>>>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result =
            self.invoke_wrapper_raw(wrapper, uri, method, args, env, resolution_context)?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }

    pub fn invoke<T: DeserializeOwned>(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self.invoke_raw(uri, method, args, env, resolution_context)?;

        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }
}

impl Invoker for PolywrapClient {
    fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<Mutex<Box<dyn Wrapper>>>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper
            .lock()
            .unwrap()
            .invoke(
                Arc::new(self.clone()),
                uri,
                method,
                args,
                env,
                resolution_context,
            )
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(result)
    }

    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let mut resolution_context = match resolution_context {
            None => &mut empty_res_context,
            Some(ctx) => ctx,
        };

        let wrapper = self
            .load_wrapper(uri, Some(&mut resolution_context))
            .map_err(|e| Error::LoadWrapperError(e.to_string()))?;

        let mut env = env;
        if env.is_none() {
            if let Some(e) = self.get_env_by_uri(&uri.clone()) {
                let e = e.to_owned();
                env = Some(e);
            };
        }

        let invoke_result = self
            .invoke_wrapper_raw(wrapper, uri, method, args, env, Some(resolution_context))
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(invoke_result)
    }

    fn get_implementations(&self, uri: Uri) -> Result<Vec<Uri>, Error> {
        polywrap_core::resolvers::helpers::get_implementations(
            uri,
            self.get_interfaces(),
            Box::new(self.clone()),
        )
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        if let Some(interfaces) = self.interfaces.clone() {
            return Some(interfaces);
        }

        None
    }
}

impl Client for PolywrapClient {
    fn get_config(&self) -> &ClientConfig {
        todo!()
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

impl Loader for PolywrapClient {
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<Mutex<Box<dyn Wrapper>>>, Error> {
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
                "Failed to resolve wrapper: {}",
                uri
            ))),
            UriPackageOrWrapper::Wrapper(_, wrapper) => Ok(wrapper),
            UriPackageOrWrapper::Package(_, package) => {
                let wrapper = package
                    .lock()
                    .unwrap()
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

    fn get_invoker(&self) -> Result<Arc<dyn Invoker>, Error> {
        Ok(Arc::new(self.clone()))
    }
}
