
use std::{fmt::Debug, sync::{Arc, Mutex}};

use polywrap_client::core::{resolution::{uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}}, invoker::Invoker};

use crate::{invoker::{FFIInvoker, InvokerWrapping}, uri::FFIUri};

use super::{uri_package_or_wrapper::{FFIUriPackageOrWrapper}, resolution_context::{FFIUriResolutionContext}};

pub trait FFIUriResolver: Send + Sync + Debug {
    fn try_resolve_uri(
      &self,
      uri: Arc<FFIUri>,
      invoker: Box<dyn FFIInvoker>,
      resolution_context: Arc<FFIUriResolutionContext>
    ) -> Box<dyn FFIUriPackageOrWrapper>;
}

#[derive(Debug)]
pub struct UriResolverWrapping(pub Box<dyn FFIUriResolver>);

impl UriResolver for UriResolverWrapping {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        let ffi_resolution_context = FFIUriResolutionContext(resolution_context);
        let result = self.0.try_resolve_uri(
          Arc::new(uri.clone().into()),
          Box::new(InvokerWrapping(invoker)),
          Arc::new(ffi_resolution_context)
        );

        Ok(result.into())
    }
}

impl UriResolverWrapping {
  pub fn as_uri_resolver(self) -> Box<dyn UriResolver> {
    Box::new(self) as Box<dyn UriResolver>
  }
}