use polywrap_client::{
    core::resolution::uri_resolver::UriResolver, resolvers::recursive_resolver::RecursiveResolver,
};
use std::sync::Arc;

use crate::{error::FFIError, invoker::FFIInvoker, uri::FFIUri};

use super::{
    ffi_resolver::{IFFIUriResolver, FFIUriResolver},
    resolution_context::FFIUriResolutionContext,
    uri_package_or_wrapper::FFIUriPackageOrWrapper,
};

#[derive(Debug)]
pub struct FFIRecursiveUriResolver(RecursiveResolver);

impl FFIRecursiveUriResolver {
    pub fn new(uri_resolver_like: Arc<FFIUriResolver>) -> FFIRecursiveUriResolver {
        FFIRecursiveUriResolver(
          RecursiveResolver::new(uri_resolver_like)
        )
    }

    pub fn try_resolve_uri(
      &self,
      uri: Arc<FFIUri>,
      invoker: Arc<FFIInvoker>,
      resolution_context: Arc<FFIUriResolutionContext>,
  ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
      IFFIUriResolver::try_resolve_uri(self, uri, invoker, resolution_context)
  }
}

impl IFFIUriResolver for FFIRecursiveUriResolver {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
        let result = self.0.try_resolve_uri(
            &uri.0,
            invoker.0.clone(),
            resolution_context.0.clone(),
        )?;

        Ok(Arc::new(FFIUriPackageOrWrapper(result)))
    }
}
