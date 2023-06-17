use crate::cache::basic_wrapper_cache::BasicWrapperCache;
use crate::cache::wrapper_cache::WrapperCache;
use crate::uri_resolver_aggregator::UriResolverAggregator;
use polywrap_core::{
    error::Error,
    invoker::Invoker,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
        uri_resolver::UriResolver,
    },
    uri::Uri,
    wrapper::Wrapper,
};
use std::fmt;
use std::sync::{Arc, Mutex};

/// A URI resolver that uses a cache to store and retrieve wrappers that pass through.
pub struct WrapperCacheResolver {
    resolver: Arc<dyn UriResolver>,
    cache: Mutex<Box<dyn WrapperCache>>,
}

impl WrapperCacheResolver {
    /// Creates a new `WrapperCacheResolver`.
    ///
    /// # Arguments
    ///
    /// * `resolver` - The `UriResolver` to use when resolving URIs.
    /// * `cache` - The cache to store and retrieve resolved URIs.
    ///
    /// # Returns
    ///
    /// * A new `WrapperCacheResolver`.
    pub fn new(
        resolver: Arc<dyn UriResolver>,
        cache: Mutex<Box<dyn WrapperCache>>,
    ) -> WrapperCacheResolver {
        WrapperCacheResolver { resolver, cache }
    }

    fn cache_resolution_path(
        &self,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
        wrapper: Arc<dyn Wrapper>,
    ) {
        let resolution_path = resolution_context.lock().unwrap().get_resolution_path();
        for uri in resolution_path {
            self.cache.lock().unwrap().set(uri, wrapper.clone());
        }
    }
}

impl UriResolver for WrapperCacheResolver {
    /// Tries to resolve the given URI using a cache and returns the result.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI to resolve.
    /// * `invoker` - The invoker of the resolution.
    /// * `resolution_context` - The context for the resolution.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the resolved `UriPackageOrWrapper` on success, or an exception on failure.
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        if let Some(wrapper) = self.cache.lock().unwrap().get(uri) {
            let result = Ok(UriPackageOrWrapper::Wrapper(uri.clone(), wrapper.clone()));
            resolution_context
                .lock()
                .unwrap()
                .track_step(UriResolutionStep {
                    source_uri: uri.clone(),
                    result: result.clone(),
                    sub_history: None,
                    description: Some("WrapperCacheResolver (Cache)".to_string()),
                });
            return result;
        }

        let sub_context = resolution_context
            .lock()
            .unwrap()
            .create_sub_history_context();
        let sub_context = Arc::new(Mutex::new(sub_context));
        let result = self
            .resolver
            .try_resolve_uri(uri, invoker.clone(), sub_context.clone());

        if result.is_ok() {
            if let UriPackageOrWrapper::Wrapper(_, wrapper) = result.clone().unwrap() {
                self.cache_resolution_path(sub_context.clone(), wrapper.clone());
            }
        }

        resolution_context
            .lock()
            .unwrap()
            .track_step(UriResolutionStep {
                source_uri: uri.clone(),
                result: result.clone(),
                sub_history: Some(sub_context.lock().unwrap().get_history().clone()),
                description: Some("WrapperCacheResolver".to_string()),
            });

        return result;
    }
}

impl fmt::Debug for WrapperCacheResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WrapperCacheResolver")
    }
}

impl From<Vec<Box<dyn UriResolver>>> for WrapperCacheResolver {
    fn from(resolvers: Vec<Box<dyn UriResolver>>) -> Self {
        WrapperCacheResolver::from(UriResolverAggregator::from(resolvers))
    }
}

impl From<UriResolverAggregator> for WrapperCacheResolver {
    fn from(resolver: UriResolverAggregator) -> Self {
        WrapperCacheResolver::new(
            Arc::new(resolver),
            Mutex::new(Box::new(BasicWrapperCache::new())),
        )
    }
}

impl From<Box<dyn UriResolver>> for WrapperCacheResolver {
    fn from(resolver: Box<dyn UriResolver>) -> Self {
        WrapperCacheResolver::new(
            Arc::from(resolver),
            Mutex::new(Box::new(BasicWrapperCache::new())),
        )
    }
}
