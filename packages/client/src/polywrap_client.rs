use async_trait::async_trait;
use polywrap_core::{
    client::{Client, ClientConfig, UriRedirect},
    error::Error,
    invoke::{InvokeOptions, Invoker},
    loader::Loader,
    uri::Uri,
    uri_resolution_context::UriResolutionContext,
    uri_resolver::{UriResolver, UriResolverHandler},
    wrapper::Wrapper,
};
use polywrap_msgpack::{decode, DeserializeOwned};

use crate::{wrapper_invoker::WrapperInvoker, wrapper_loader::WrapperLoader};

pub struct PolywrapClient {
    config: ClientConfig,
    loader: WrapperLoader,
    invoker: WrapperInvoker,
}

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        let loader = WrapperLoader::new(config.resolver.clone());
        let invoker = WrapperInvoker::new(loader.clone());

        Self {
            config,
            invoker,
            loader,
        }
    }

    pub async fn invoke_wrapper_and_decode<T: DeserializeOwned>(
        &self,
        options: &InvokeOptions<'_>,
        wrapper: Box<dyn Wrapper>,
    ) -> Result<T, Error> {
        let result = self.invoke_wrapper(options, wrapper).await?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }

    pub async fn invoke_and_decode<T: DeserializeOwned>(
        &self,
        options: &InvokeOptions<'_>,
    ) -> Result<T, Error> {
        let result = self.invoke(options).await?;
        decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e)))
    }
}

#[async_trait]
impl Invoker for PolywrapClient {
    async fn invoke(&self, options: &InvokeOptions) -> Result<Vec<u8>, Error> {
        self.invoker.invoke(options).await
    }

    async fn invoke_wrapper(
        &self,
        options: &InvokeOptions,
        wrapper: Box<dyn Wrapper>,
    ) -> Result<Vec<u8>, Error> {
        self.invoker.invoke_wrapper(options, wrapper).await
    }
}

#[async_trait(?Send)]
impl Client for PolywrapClient {
    fn get_config(&self) -> &ClientConfig {
        &self.config
    }

    fn get_redirects(&self) -> &Vec<UriRedirect> {
        &self.config.redirects
    }

    fn get_uri_resolver(&self) -> &dyn UriResolver {
        self.config.resolver.as_ref()
    }

    // async fn get_file(&self, uri: &Uri, options: &GetFileOptions) -> Result<Vec<u8>, Error> {
    //     let load = self.load_wrapper(uri, Option::None).await;

    //     match load {
    //         Ok(wrapper) => {
    //             let result = wrapper.get_file(options);
    //             return result;
    //         }
    //         Err(err) => {
    //             return Err(Error::GetFileError(format!(
    //                 "Failed to load wrapper: {}",
    //                 err
    //             )));
    //         }
    //     }
    // }
}

#[async_trait]
impl UriResolverHandler for PolywrapClient {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&UriResolutionContext>,
    ) -> Result<polywrap_core::uri_resolution_context::UriPackageOrWrapper, Error> {
        self.loader.try_resolve_uri(uri, resolution_context).await
    }
}

#[async_trait]
impl Loader for PolywrapClient {
    async fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&UriResolutionContext>,
    ) -> Result<Box<dyn Wrapper>, Error> {
        self.loader.load_wrapper(uri, resolution_context).await
    }
}
