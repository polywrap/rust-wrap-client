use std::{sync::Arc, ops::DerefMut};
use polywrap_client::{resolvers::recursive_resolver::RecursiveResolver, core::{resolution::{uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}}, invoker::Invoker}};

use crate::{uri::FFIUri, invoker::FFIInvoker};

use super::{ffi_resolver::{FFIUriResolver, ExtUriResolver}, resolution_context::FFIUriResolutionContext, uri_package_or_wrapper::FFIUriPackageOrWrapper};

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

  pub fn try_resolve_uri(
    &self,
    uri: Arc<FFIUri>,
    client: Arc<FFIInvoker>,
    resolution_context: Arc<FFIUriResolutionContext>
  ) -> Box<dyn FFIUriPackageOrWrapper> {
    let mut uri_res_ctx_guard = resolution_context.0.lock().unwrap();

    let result = self.inner_resolver.try_resolve_uri(
      &uri.0,
      client,
      uri_res_ctx_guard.deref_mut()
    ).unwrap();

    Box::new(result)
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