use polywrap_client::{
    core::{
        resolution::{
            uri_resolver::UriResolver,
        }
    },
    resolvers::recursive_resolver::RecursiveResolver,
};
use std::{sync::Arc};

use crate::{uri::FFIUri, invoker::{FFIInvoker, FFIInvokerWrapping}};

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
        invoker: Box<dyn FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Box<dyn FFIUriPackageOrWrapper> {

        let result = self
            .inner_resolver
            .try_resolve_uri(&uri.0, Arc::new(FFIInvokerWrapping(invoker)), resolution_context.0.clone())
            .unwrap();

        Box::new(result)
    }
}
