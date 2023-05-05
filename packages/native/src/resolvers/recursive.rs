use std::sync::Arc;

use polywrap_client::{core::{resolvers::{uri_resolution_context::UriPackageOrWrapper, uri_resolver::UriResolver, recursive_resolver::RecursiveResolver, uri_resolver_like::UriResolverLike}, invoke::Invoker}};
use super::{uri_resolver_like::FFIUriResolverLike};

#[derive(Debug)]
pub struct FFIRecursiveUriResolver {
  inner_resolver: RecursiveResolver
}

impl FFIRecursiveUriResolver {
  pub fn new(uri_resolver_like: Arc<FFIUriResolverLike>) -> FFIRecursiveUriResolver {
    let uri_resolver_like: UriResolverLike = uri_resolver_like.as_ref().clone().into();
    
    FFIRecursiveUriResolver {
      inner_resolver: RecursiveResolver::from(uri_resolver_like)
    }
  }
}

impl UriResolver for FFIRecursiveUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        self.inner_resolver.try_resolve_uri(uri, invoker, resolution_context)
    }
}