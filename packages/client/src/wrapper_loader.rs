use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    loader::Loader,
    uri::Uri,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri_resolver::{UriResolver, UriResolverHandler},
    wrapper::Wrapper,
};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct WrapperLoader {
    uri_resolver: Arc<Mutex<dyn UriResolver>>,
}

impl WrapperLoader {
    pub fn new(uri_resolver: Arc<Mutex<dyn UriResolver>>) -> Self {
        Self { uri_resolver }
    }
}

#[async_trait]
impl UriResolverHandler for WrapperLoader {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_resolver = self.uri_resolver.clone();
        let mut uri_resolver_context = UriResolutionContext::new();

        let mut resolution_context = match resolution_context {
            Some(ctx) => ctx,
            None => &mut uri_resolver_context,
        };

        let x = uri_resolver.lock().await
            .try_resolve_uri(uri, self, &mut resolution_context)
            .await; x
    }
}

#[async_trait]
impl Loader for WrapperLoader {
    async fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Box<dyn Wrapper>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let mut resolution_ctx = match resolution_context {
            Some(ctx) => ctx,
            None => &mut empty_res_context,
        };

        let uri_package_or_wrapper = self
            .try_resolve_uri(uri, Some(&mut resolution_ctx))
            .await
            .map_err(|e| Error::ResolutionError(e.to_string()))?;

        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri) => Err(Error::InvokeError(format!(
                "Failed to resolve wrapper: {}",
                uri
            ))),
            UriPackageOrWrapper::Wrapper(_, wrapper) => Ok(wrapper),
            UriPackageOrWrapper::Package(_, package) => {
                let wrapper = package
                    .create_wrapper()
                    .await
                    .map_err(|e| Error::WrapperCreateError(e.to_string()))?;
                Ok(wrapper)
            }
        }
    }
}
