use std::collections::HashMap;
use std::sync::Arc;
use polywrap_core::uri::Uri;
use polywrap_core::wrapper::Wrapper;
use crate::cache::wrapper_cache::WrapperCache;

/// A simple cache for storing `Wrapper` instances.
pub struct BasicWrapperCache {
    cache: HashMap<Uri, Arc<dyn Wrapper>>,
}

impl BasicWrapperCache {
    /// Creates a new `BasicWrapperCache`.
    ///
    /// # Returns
    ///
    /// * A new `BasicWrapperCache`.
    pub fn new() -> BasicWrapperCache {
        BasicWrapperCache {
            cache: HashMap::new(),
        }
    }
}

impl WrapperCache for BasicWrapperCache {
    /// Gets the `Wrapper` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to get the `Wrapper` for.
    ///
    /// # Returns
    ///
    /// * The `Wrapper` instance for the given `Uri`, or None if it does not exist.
    fn get(&self, uri: &Uri) -> Option<&Arc<dyn Wrapper>> {
        self.cache.get(uri)
    }

    /// Sets the `Wrapper` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to set the `Wrapper` for.
    /// * `wrapper` - The `Wrapper` instance to set.
    fn set(&mut self, uri: Uri, wrapper: Arc<dyn Wrapper>) {
        self.cache.insert(uri, wrapper);
    }
}
