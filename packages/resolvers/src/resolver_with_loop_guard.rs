use core::fmt;
use std::sync::Arc;

use polywrap_core::{
    error::Error,
    invoker::Invoker,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::UriResolver,
    },
    uri::Uri,
};

pub struct ResolverWithLoopGuard {
    pub resolver: Arc<dyn UriResolver>,
}

impl UriResolver for ResolverWithLoopGuard {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if resolution_context.is_resolving(uri) {
            //TODO handle this error
            Err(Error::ResolverError("Infinite Loop".to_string()))
        } else {
            resolution_context.start_resolving(uri);

            let result = self
                .resolver
                .try_resolve_uri(uri, invoker, resolution_context);

            resolution_context.stop_resolving(uri);

            result
        }
    }
}

impl fmt::Debug for ResolverWithLoopGuard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResolverWithLoopGuard")
    }
}
