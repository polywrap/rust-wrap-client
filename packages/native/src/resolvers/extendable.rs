use std::{sync::Arc, ops::DerefMut};

use polywrap_client::{resolvers::extendable_uri_resolver::ExtendableUriResolver, core::{resolution::{uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}}, invoker::Invoker}};

use crate::{invoker::FFIInvoker, uri::FFIUri};

use super::{uri_package_or_wrapper::FFIUriPackageOrWrapper, resolution_context::FFIUriResolutionContext};

#[derive(Debug)]
pub struct FFIExtendableUriResolver {
  inner_resolver: ExtendableUriResolver
}

impl FFIExtendableUriResolver {
  pub fn new(name: Option<String>) -> FFIExtendableUriResolver {
    FFIExtendableUriResolver {
      inner_resolver: ExtendableUriResolver::new(name)
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

impl UriResolver for FFIExtendableUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        self.inner_resolver.try_resolve_uri(uri, invoker, resolution_context)
    }
}