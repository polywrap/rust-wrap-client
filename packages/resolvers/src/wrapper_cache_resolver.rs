use std::fmt;
use std::sync::{Arc, Mutex};
use polywrap_core::{
    invoker::Invoker,
    uri::Uri,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::UriResolver
    },
    error::Error,
};
use polywrap_core::resolution::uri_resolution_context::UriResolutionStep;
use crate::cache::wrapper_cache::WrapperCache;

/// A URI resolver that uses a cache to store and retrieve the results of resolved URIs.
pub struct WrapperCacheResolver {
    resolver: Arc<dyn UriResolver>,
    cache: Mutex<Box<dyn WrapperCache>>,
}

impl WrapperCacheResolver {
    /// Creates a new `CacheResolver`.
    ///
    /// # Arguments
    ///
    /// * `resolver` - The `UriResolver` to use when resolving URIs.
    /// * `cache` - The cache to store and retrieve resolved URIs.
    ///
    /// # Returns
    ///
    /// * A new `CacheResolver`.
    pub fn new(resolver: Arc<dyn UriResolver>, cache: Mutex<Box<dyn WrapperCache>>) -> WrapperCacheResolver {
        WrapperCacheResolver { resolver, cache }
    }

    /// Caches the result of a resolved URI based on its type.
    ///
    /// # Arguments
    ///
    /// * `uri_package_or_wrapper` - The resolved URI to cache.
    /// * `sub_context` - The context for the resolution.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the cached `UriPackageOrWrapper` on success, or an error on failure.
    fn cache_result(
        &self,
        uri_package_or_wrapper: UriPackageOrWrapper,
        sub_context: Arc<Mutex<UriResolutionContext>>
    ) -> Result<UriPackageOrWrapper, Error> {
        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri_value) => {
                Ok(UriPackageOrWrapper::Uri(uri_value))
            }

            UriPackageOrWrapper::Package(resolved_uri, wrap_package) => {
                match wrap_package.create_wrapper() {
                    Err(e) => Err(e),
                    Ok(wrapper) => {
                        let resolution_path = sub_context.lock().unwrap().get_resolution_path();
                        for uri in resolution_path {
                            self.cache.lock().unwrap().set(uri, wrapper.clone());
                        }
                        Ok(UriPackageOrWrapper::Wrapper(resolved_uri, wrapper))
                    }
                }
            }

            UriPackageOrWrapper::Wrapper(resolved_uri, wrapper) => {
                let resolution_path = sub_context.lock().unwrap().get_resolution_path();
                for uri in resolution_path {
                    self.cache.lock().unwrap().set(uri, wrapper.clone());
                }

                Ok(UriPackageOrWrapper::Wrapper(resolved_uri, wrapper))
            }
        }
    }

    // fn cache_resolution_path(&mut self, resolution_path: Vec<Uri>, wrapper: &Box<dyn Wrapper>) {
    //     for uri in resolution_path {
    //         self.cache.set(uri, wrapper.clone());
    //     }
    // }
}

impl fmt::Debug for WrapperCacheResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WrapperCacheResolver")
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
            resolution_context.lock().unwrap().track_step(
                UriResolutionStep {
                    source_uri: uri.clone(),
                    result: result.clone(),
                    sub_history: None,
                    description: Some("CacheResolver (Cache)".to_string()),
                }
            );
            return result;
        }

        let sub_context = resolution_context.lock().unwrap().create_sub_history_context();
        let sub_context = Arc::new(Mutex::new(sub_context));
        let result = self.resolver.try_resolve_uri(uri, invoker.clone(), sub_context.clone());
        let final_result = match result {
            Ok(uri_package_or_wrapper) => self.cache_result(uri_package_or_wrapper, sub_context.clone()),
            Err(_) => result,
        };

        resolution_context.lock().unwrap().track_step(
            UriResolutionStep {
                source_uri: uri.clone(),
                result: final_result.clone(),
                sub_history: Some(sub_context.lock().unwrap().get_history().clone()),
                description: Some("CacheResolver".to_string()),
            }
        );

        return final_result;
    }
}
