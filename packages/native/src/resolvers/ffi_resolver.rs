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
    resolution_context::FFIUriResolutionContext, uri_package_or_wrapper::FFIUriPackageOrWrapper,
};

pub trait IFFIUriResolver: Send + Sync + Debug {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError>;
}

impl IFFIUriResolver for Arc<dyn UriResolver> {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
        let uri_package_or_wrapper = UriResolver::try_resolve_uri(
            self.as_ref(),
            &uri.0,
            invoker.0.clone(),
            resolution_context.0.clone(),
        )?;

        Ok(Arc::new(FFIUriPackageOrWrapper(uri_package_or_wrapper)))
    }
}

#[derive(Debug)]
pub struct FFIUriResolver(pub Box<dyn IFFIUriResolver>);

impl FFIUriResolver {
    pub fn new(uri_resolver: Box<dyn IFFIUriResolver>) -> Self {
        Self(uri_resolver)
    }

    pub fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
        self.0.try_resolve_uri(uri, invoker, resolution_context)
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
        let result = self.0.try_resolve_uri(
            Arc::new(uri.clone().into()),
            Arc::new(FFIInvoker(invoker)),
            Arc::new(ffi_resolution_context),
        )?;

        Ok(result.as_ref().0.clone())
    }
}
