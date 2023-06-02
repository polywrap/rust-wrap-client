use polywrap_client::{
    core::{
        resolution::{
            uri_resolver::UriResolver,
        }
    },
    resolvers::recursive_resolver::RecursiveResolver,
};
use std::{ops::DerefMut, sync::Arc};

use crate::{invoker::FFIInvoker, uri::FFIUri};

use super::{
    ffi_resolver::{FFIUriResolver, UriResolverWrapping}, resolution_context::FFIUriResolutionContext,
    uri_package_or_wrapper::FFIUriPackageOrWrapper,
};

#[derive(Debug)]
pub struct FFIRecursiveUriResolver {
    inner_resolver: RecursiveResolver,
}

impl FFIRecursiveUriResolver {
    pub fn new(uri_resolver_like: Box<dyn FFIUriResolver>) -> FFIRecursiveUriResolver {
        FFIRecursiveUriResolver {
            inner_resolver: (UriResolverWrapping(uri_resolver_like).as_uri_resolver()).into(),
        }
    }
}

impl FFIUriResolver for FFIRecursiveUriResolver {
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
