use async_trait::async_trait;

use polywrap_core::{
    client::{Client, ClientConfig, UriRedirect},
    error::CoreError,
    uri::{
        uri::Uri,
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::{TryResolveUriOptions, UriResolver, UriResolverHandler},
    },
    wrapper::{GetFileOptions, Wrapper},
};

use crate::error::ClientError;

pub struct PolywrapClient {
    config: ClientConfig,
}

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub async fn load_wrapper(
        &mut self,
        uri: &Uri,
        resolution_context: Option<UriResolutionContext>,
    ) -> Result<Box<dyn Wrapper>, ClientError> {
        let resolution_ctx = match resolution_context {
            Some(ctx) => ctx,
            None => UriResolutionContext::new(),
        };

        let result = self
            .try_resolve_uri(&TryResolveUriOptions {
                uri: uri.clone(),
                resolution_context: Some(resolution_ctx),
            })
            .await;

        // TODO: Handle errors
        if result.is_err() {
            return Err(ClientError::LoadWrapperError(format!(
                "Failed to resolve wrapper: {}",
                result.err().unwrap()
            )));
        };

        let uri_package_or_wrapper = result.unwrap();

        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri) => {
                return Err(ClientError::LoadWrapperError(format!(
                    "Failed to resolve wrapper: {}",
                    uri
                )));
            }
            UriPackageOrWrapper::Wrapper(_, wrapper) => {
                return Ok(wrapper.wrapper);
            }
            UriPackageOrWrapper::Package(_, package) => {
                let wrapper = package.package.create_wrapper().await.unwrap();
                return Ok(wrapper);
            }
        }
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

    fn get_uri_resolver(&self) -> &Box<dyn UriResolver> {
        &self.config.resolver
    }

    async fn get_file(&mut self, uri: Uri, options: GetFileOptions) -> Result<String, CoreError> {
        let load = self.load_wrapper(&uri, Option::None).await;

        match load {
            Ok(wrapper) => {
                let result = wrapper.get_file(&options).await;
                return result;
            }
            Err(err) => {
                return Err(CoreError::GetFileError(format!(
                    "Failed to load wrapper: {}",
                    err
                )));
            }
        }
    }
}

#[async_trait(?Send)]
impl UriResolverHandler for PolywrapClient {
    async fn try_resolve_uri(
        &mut self,
        options: &polywrap_core::uri::uri_resolver::TryResolveUriOptions,
    ) -> Result<polywrap_core::uri::uri_resolution_context::UriPackageOrWrapper, CoreError> {
        let uri = options.uri.clone();

        let uri_resolver = self.get_uri_resolver();
        let uri_resolver_context = UriResolutionContext::new();

        let resolution_context = match &options.resolution_context {
            Some(ctx) => ctx,
            None => &uri_resolver_context,
        };

        uri_resolver.try_resolve_uri(&uri, Box::new(self), resolution_context).await
    }
}
