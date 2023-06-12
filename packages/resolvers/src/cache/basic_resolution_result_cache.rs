use std::collections::HashMap;
use std::sync::Arc;
use polywrap_core::error::Error;
use polywrap_core::resolution::uri_resolution_context::UriPackageOrWrapper;
use polywrap_core::uri::Uri;
use crate::cache::resolution_result_cache::ResolutionResultCache;

/// A simple cache for storing `Result<UriPackageOrWrapper, Error>` instances.
pub struct BasicResolutionResultCache {
    cache: HashMap<Uri, Arc<Result<UriPackageOrWrapper, Error>>>,
}

impl BasicResolutionResultCache {
    /// Creates a new `BasicResolutionResultCache`.
    ///
    /// # Returns
    ///
    /// * A new `BasicResolutionResultCache`.
    pub fn new() -> BasicResolutionResultCache {
        BasicResolutionResultCache {
            cache: HashMap::new(),
        }
    }
}

impl ResolutionResultCache for BasicResolutionResultCache {
    /// Gets the `Result<UriPackageOrWrapper, Error>` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to get the `Wrapper` for.
    ///
    /// # Returns
    ///
    /// * The `Result<UriPackageOrWrapper, Error>` instance for the given `Uri`, or None if it does not exist.
    fn get(&self, uri: &Uri) -> Option<&Arc<Result<UriPackageOrWrapper, Error>>> {
        self.cache.get(uri)
    }

    /// Sets the `Result<UriPackageOrWrapper, Error>` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to set the `Result<UriPackageOrWrapper, Error>` for.
    /// * `result` - The `Result<UriPackageOrWrapper, Error>` instance to set.
    fn set(&mut self, uri: Uri, result: Arc<Result<UriPackageOrWrapper, Error>>) {
        self.cache.insert(uri, result);
    }
}
