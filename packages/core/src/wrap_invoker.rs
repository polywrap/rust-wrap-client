use crate::{
    error::Error, resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
    wrapper::Wrapper,
};

/// Defines an object capable of invoking wrappers
pub trait WrapInvoker: Send + Sync {
    /// Invokes a method on a given wrapper.
    /// The method returns a Result containing either the msgpcack buffer of the response or an Error.
    ///
    /// # Arguments
    ///
    /// * `wrapper` - The wrapper to invoke
    /// * `uri` - URI of the invoked wrapper.
    /// * `method` - The name of the method to invoke.
    /// * `args` - Optional msgpack buffer representing the arguments to the method.
    /// * `env` - Optional msgpack buffer representing the environment for the method.
    /// * `resolution_context` - Optional resolution context of invocation.
    fn invoke_wrapper_raw(
        &self,
        wrapper: &dyn Wrapper,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
}
