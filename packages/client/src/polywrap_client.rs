use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    client::{Client, ClientConfig},
    error::Error,
    invoke::{Invoker, InvokeArgs},
    loader::Loader,
    uri::Uri,
    resolvers::uri_resolution_context::UriResolutionContext,
    resolvers::uri_resolver::{UriResolverHandler},
    wrapper::Wrapper, env::{Env},
    interface_implementation::InterfaceImplementations
};
use polywrap_msgpack::{decode, DeserializeOwned};
use tokio::sync::Mutex;

use crate::{wrapper_invoker::WrapperInvoker, wrapper_loader::WrapperLoader};

#[derive(Clone)]
pub struct PolywrapClient {
    pub loader: WrapperLoader,
    invoker: WrapperInvoker
}

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        let resolver = config.resolver;
        let loader = WrapperLoader::new(
            resolver, 
            config.envs.clone(),
            config.interfaces.clone()
        );
        let invoker = WrapperInvoker::new(loader.clone());

        Self {
            invoker,
            loader
        }
    }

    pub async fn invoke_wrapper_and_decode<T: DeserializeOwned>(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self
            .invoke_wrapper(wrapper, uri, method, args, env, resolution_context)
            .await?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }

    pub async fn invoke_and_decode<T: DeserializeOwned>(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<T, Error> {
        let result = self.invoke(uri, method, args, env, resolution_context).await?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }
}

#[async_trait]
impl Invoker for PolywrapClient {
    async fn invoke(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let env_uri = match env {
            Some(env) => Some(env),
            None => {
                self.loader.get_env_by_uri(uri).map(|env| env.to_owned())
            }
        };
        self.invoker.invoke(uri, method, args, env_uri, resolution_context).await
    }

    async fn invoke_wrapper(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        self.invoker.invoke_wrapper(wrapper, uri, method, args, env, resolution_context).await
    }

    async fn get_implementations(&self, uri: Uri) -> Result<Vec<Uri>, Error> {
        self.invoker.get_implementations(uri).await
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        self.invoker.get_interfaces()
    }
}

#[async_trait(?Send)]
impl Client for PolywrapClient {
    fn get_config(&self) -> &ClientConfig {
        todo!()
    }
}

#[async_trait]
impl UriResolverHandler for PolywrapClient {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<polywrap_core::resolvers::uri_resolution_context::UriPackageOrWrapper, Error> {
        self.loader.try_resolve_uri(uri, resolution_context).await
    }
}

#[async_trait]
impl Loader for PolywrapClient {
    async fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<Mutex<dyn Wrapper>>, Error> {
        self.loader.load_wrapper(uri, resolution_context).await
    }

    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env> {
        self.loader.get_env_by_uri(uri)
    }
    
    fn get_invoker(&self) -> Result<Arc<dyn Invoker>, Error>  {
        self.loader.get_invoker()
    }
}
