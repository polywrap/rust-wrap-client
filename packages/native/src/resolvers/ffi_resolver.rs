use polywrap_client::core::resolvers::uri_resolver::UriResolver;

use crate::loader::FFILoader;
use std::fmt::Debug;

use super::uri_package_or_wrapper::FFIUriPackageOrWrapper;

pub trait FFIUriResolver: Send + Sync + Debug {
    fn ffi_try_resolve_uri(
      &self,
      uri: &str,
      loader: FFILoader
    ) -> FFIUriPackageOrWrapper;
}

#[derive(Debug)]
pub struct FFIUriResolverWrapper(Box<dyn FFIUriResolver>);

impl UriResolver for FFIUriResolverWrapper {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        loader: std::sync::Arc<dyn polywrap_client::core::loader::Loader>,
        _: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<polywrap_client::core::resolvers::uri_resolution_context::UriPackageOrWrapper, polywrap_client::core::error::Error> {
        let loader = FFILoader::new(loader);
        Ok(self.0._try_resolve_uri(Arc::new(uri.clone()), loader).into())
    }
}

impl From<Box<dyn FFIUriResolver>> for FFIUriResolverWrapper {
    fn from(value: Box<dyn FFIUriResolver>) -> Self {
        FFIUriResolverWrapper(value)
    }
}