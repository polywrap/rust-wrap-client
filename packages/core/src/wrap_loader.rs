use std::sync::{Arc, Mutex};

use crate::{
    error::Error, resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
    wrapper::Wrapper,
};

/// Capable of loading wrappers from URIs.
pub trait WrapLoader: Send + Sync {
    /// Loads a wrapper from a given URI. On success, returns a `Wrapper`. On failure, returns an `Error`
    ///
    /// # Arguments
    /// - `uri`: The `Uri` from which to load the wrapper.
    /// - `resolution_context`: An optional TODO.
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Arc<dyn Wrapper>, Error>;
}
