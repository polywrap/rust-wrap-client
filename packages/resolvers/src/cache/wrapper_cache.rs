use std::sync::Arc;
use polywrap_core::uri::Uri;
use polywrap_core::wrapper::Wrapper;

/// A cache for storing `Wrapper` instances.
pub trait WrapperCache : Send + Sync {
    /// Gets the `Wrapper` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to get the `Wrapper` for.
    ///
    /// # Returns
    ///
    /// * The `Wrapper` instance for the given `Uri`, or None if it does not exist.
    fn get(&self, uri: &Uri) -> Option<&Arc<dyn Wrapper>>;

    /// Sets the `Wrapper` instance for the given `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` to set the `Wrapper` for.
    /// * `wrapper` - The `Wrapper` instance to set.
    fn set(&mut self, uri: Uri, wrapper: Arc<dyn Wrapper>);
}
