use polywrap_core::error::Error;
use polywrap_core::resolution::uri_resolution_context::UriPackageOrWrapper;
use polywrap_core::uri::Uri;
use std::sync::Arc;

/// A cache for storing `Wrapper` instances.
pub trait ResolutionResultCache: Send + Sync {
    /// Gets the `Wrapper` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to get the `Wrapper` for.
    ///
    /// # Returns
    ///
    /// * The `Wrapper` instance for the given `Uri`, or None if it does not exist.
    fn get(&self, uri: &Uri) -> Option<&Arc<Result<UriPackageOrWrapper, Error>>>;

    /// Sets the `Wrapper` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to set the `Wrapper` for.
    /// * `wrapper` - The `Wrapper` instance to set.
    fn set(&mut self, uri: Uri, wrapper: Arc<Result<UriPackageOrWrapper, Error>>);
}
