
use std::{fmt::Debug, sync::{Arc, Mutex}};

use polywrap_client::core::{resolution::{uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}}, invoker::Invoker};

use crate::{invoker::FFIInvoker, uri::FFIUri};

use super::{uri_package_or_wrapper::{FFIUriPackageOrWrapper}, resolution_context::{FFIUriResolutionContext}};

pub trait FFIUriResolver: Send + Sync + Debug {
    fn try_resolve_uri(
      &self,
      uri: Arc<FFIUri>,
      client: Arc<FFIInvoker>,
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
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        // Create FFIUriResolutionContext from resolution_context clone
        let ffi_resolution_context = Arc::new(Mutex::new((resolution_context.clone()).into()));
        let result = self.0.try_resolve_uri(
          Arc::new(uri.clone().into()),
          Arc::new(FFIInvoker::new(invoker)),
          Arc::new(FFIUriResolutionContext(ffi_resolution_context.clone()))
        );

        // Update resolution_context from FFIUriResolutionContext clone
        *resolution_context = ffi_resolution_context.lock().unwrap().clone();

        Ok(result.into())
    }
}

impl UriResolverWrapping {
  pub fn as_uri_resolver(self) -> Box<dyn UriResolver> {
    Box::new(self) as Box<dyn UriResolver>
  }
}