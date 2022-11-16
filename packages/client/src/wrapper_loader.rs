use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    loader::Loader,
    uri::Uri,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri_resolver::{UriResolver, UriResolverHandler},
    wrapper::Wrapper, interface_implementation::{InterfaceImplementations},
};

#[derive(Clone)]
pub struct WrapperLoader {
    uri_resolver: Arc<dyn UriResolver>,
}

impl WrapperLoader {
    pub fn new(uri_resolver: Arc<dyn UriResolver>) -> Self {
        Self { uri_resolver }
    }
}

#[async_trait]
impl UriResolverHandler for WrapperLoader {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_resolver = self.uri_resolver.clone();
        let uri_resolver_context = UriResolutionContext::new();

        let resolution_context = match resolution_context {
            Some(ctx) => ctx,
            None => &uri_resolver_context,
        };

        uri_resolver
            .try_resolve_uri(uri, self, resolution_context)
            .await
    }
}

#[async_trait]
impl Loader for WrapperLoader {
    async fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&UriResolutionContext>,
    ) -> Result<Box<dyn Wrapper>, Error> {
        let empty_res_context = UriResolutionContext::new();
        let resolution_ctx = match resolution_context {
            Some(ctx) => ctx,
            None => &empty_res_context,
        };

        let uri_package_or_wrapper = self
            .try_resolve_uri(uri, Some(resolution_ctx))
            .await
            .map_err(|e| Error::ResolutionError(e.to_string()))?;

        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri) => Err(Error::InvokeError(format!(
                "Failed to resolve wrapper: {}",
                uri
            ))),
            UriPackageOrWrapper::Wrapper(_, wrapper) => Ok(wrapper.wrapper),
            UriPackageOrWrapper::Package(_, package) => {
                let wrapper = package
                    .package
                    .create_wrapper()
                    .await
                    .map_err(|e| Error::WrapperCreateError(e.to_string()))?;
                Ok(wrapper)
            }
        }
    }
}
