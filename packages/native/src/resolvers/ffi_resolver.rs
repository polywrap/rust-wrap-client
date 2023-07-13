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

pub trait FFIUriResolver: Send + Sync + Debug {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError>;
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
            Arc::new(FFIInvoker(invoker)),
            Arc::new(ffi_resolution_context),
        )?;

        Ok(result.as_ref().0.clone())
    }
}

impl UriResolverWrapping {
    pub fn as_uri_resolver(self) -> Box<dyn UriResolver> {
        Box::new(self) as Box<dyn UriResolver>
    }
}
