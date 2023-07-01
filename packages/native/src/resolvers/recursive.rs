use polywrap_client::{
    core::resolution::uri_resolver::UriResolver, resolvers::recursive_resolver::RecursiveResolver,
};
use std::sync::Arc;

use crate::{error::FFIError, invoker::FFIInvoker, uri::FFIUri};

use super::{
    ffi_resolver::{IFFIUriResolver, UriResolverWrapping},
    resolution_context::FFIUriResolutionContext,
    uri_package_or_wrapper::IFFIUriPackageOrWrapper,
};

#[derive(Debug)]
pub struct FFIRecursiveUriResolver {
    inner_resolver: RecursiveResolver,
}

impl FFIRecursiveUriResolver {
    pub fn new(uri_resolver_like: Box<dyn IFFIUriResolver>) -> FFIRecursiveUriResolver {
        FFIRecursiveUriResolver {
            inner_resolver: (UriResolverWrapping(uri_resolver_like).as_uri_resolver()).into(),
        }
    }
}

impl IFFIUriResolver for FFIRecursiveUriResolver {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Box<dyn IFFIUriPackageOrWrapper>, FFIError> {
        let result = self.inner_resolver.try_resolve_uri(
            &uri.0,
            invoker.0.clone(),
            resolution_context.0.clone(),
        )?;

        Ok(Box::new(result))
    }
}
