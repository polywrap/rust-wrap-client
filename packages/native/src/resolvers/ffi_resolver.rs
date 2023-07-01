use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use polywrap_client::core::{
    invoker::Invoker,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::UriResolver,
    },
};

use crate::{error::FFIError, invoker::FFIInvoker, uri::FFIUri};

use super::{
    resolution_context::FFIUriResolutionContext, uri_package_or_wrapper::{IFFIUriPackageOrWrapper, FFIUriPackageOrWrapper},
};

pub trait IFFIUriResolver: Send + Sync + Debug {
    fn ffi_try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Box<dyn IFFIUriPackageOrWrapper>, FFIError>;
}

#[derive(Debug)]
pub struct FFIUriResolver(pub Box<dyn IFFIUriResolver>);

impl FFIUriResolver {
  pub fn new(resolver: Box<dyn IFFIUriResolver>) -> Self {
    Self(resolver)
  }

  pub fn try_resolve_uri(
    &self,
    uri: Arc<FFIUri>,
    invoker: Arc<FFIInvoker>,
    resolution_context: Arc<FFIUriResolutionContext>,
  ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
    let result = self.0.ffi_try_resolve_uri(uri, invoker, resolution_context)?;
    Ok(Arc::new(FFIUriPackageOrWrapper(result)))
  }
}

impl UriResolver for FFIUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        let ffi_resolution_context = FFIUriResolutionContext(resolution_context);
        let result = self.0.ffi_try_resolve_uri(
            Arc::new(uri.clone().into()),
            Arc::new(FFIInvoker(invoker)),
            Arc::new(ffi_resolution_context),
        )?;

        Ok(result.into())
    }
}

impl FFIUriResolver {
    pub fn as_uri_resolver(self) -> Box<dyn UriResolver> {
        Box::new(self) as Box<dyn UriResolver>
    }
}
