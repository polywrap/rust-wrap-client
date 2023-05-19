use std::sync::Arc;
use polywrap_client::{resolvers::recursive_resolver::RecursiveResolver, core::{resolution::{uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}}, invoker::Invoker}};

use super::{ffi_resolver::{FFIUriResolver, ExtUriResolver}};

#[derive(Debug)]
pub struct FFIRecursiveUriResolver {
  inner_resolver: RecursiveResolver
}

impl FFIRecursiveUriResolver {
  pub fn new(uri_resolver_like: Box<dyn FFIUriResolver>) -> FFIRecursiveUriResolver {
    FFIRecursiveUriResolver {
      inner_resolver: (Box::new(ExtUriResolver(uri_resolver_like)) as Box<dyn UriResolver>).into()
    }
  }
}

impl UriResolver for FFIRecursiveUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        self.inner_resolver.try_resolve_uri(uri, invoker, resolution_context)
    }
}