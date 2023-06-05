use std::{sync::Arc};

use polywrap_client::{
    core::{
        resolution::{
            uri_resolver::UriResolver,
        },
    },
    resolvers::extendable_uri_resolver::ExtendableUriResolver,
};

use crate::{FFIInvokerWrapping, uri::FFIUri, invoker::FFIInvoker};

use super::{
    ffi_resolver::{FFIUriResolver}, resolution_context::FFIUriResolutionContext,
    uri_package_or_wrapper::FFIUriPackageOrWrapper,
};

#[derive(Debug)]
pub struct FFIExtendableUriResolver {
    inner_resolver: ExtendableUriResolver,
}

impl FFIExtendableUriResolver {
    pub fn new(name: Option<String>) -> FFIExtendableUriResolver {
        FFIExtendableUriResolver {
            inner_resolver: ExtendableUriResolver::new(name),
        }
    }

    pub fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Box<dyn FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Box<dyn FFIUriPackageOrWrapper> {
        let result = self
            .inner_resolver
            .try_resolve_uri(&uri.0, Arc::new(FFIInvokerWrapping::new(invoker)), resolution_context.0.clone())
            .unwrap();

        Box::new(result)
    }
}

impl FFIUriResolver for FFIExtendableUriResolver {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Box<dyn FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Box<dyn FFIUriPackageOrWrapper> {
        let result = self
            .inner_resolver
            .try_resolve_uri(&uri.0, Arc::new(FFIInvokerWrapping::new(invoker)), resolution_context.0.clone())
            .unwrap();

        Box::new(result)
    }
}
