use std::{ops::DerefMut, sync::Arc};

use polywrap_client::{
    core::{
        resolution::{
            uri_resolver::UriResolver,
        },
    },
    resolvers::extendable_uri_resolver::ExtendableUriResolver,
};

use crate::{invoker::FFIInvoker, uri::FFIUri};

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
        client: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Box<dyn FFIUriPackageOrWrapper> {
        let mut uri_res_ctx_guard = resolution_context.0.lock().unwrap();

        let result = self
            .inner_resolver
            .try_resolve_uri(&uri.0, client, uri_res_ctx_guard.deref_mut())
            .unwrap();

        Box::new(result)
    }
}

impl FFIUriResolver for FFIExtendableUriResolver {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        client: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Box<dyn FFIUriPackageOrWrapper> {
        let mut uri_res_ctx_guard = resolution_context.0.lock().unwrap();

        let result = self
            .inner_resolver
            .try_resolve_uri(&uri.0, client, uri_res_ctx_guard.deref_mut())
            .unwrap();

        Box::new(result)
    }
}
