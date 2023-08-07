use std::sync::{Arc, Mutex};

use crate::{
    error::Error, interface_implementation::InterfaceImplementations,
    resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
};
pub trait Invoker: Send + Sync {
    /// Invokes a method on a given URI with optional arguments and environment.
    /// The method returns a Result containing either the msgpcack buffer of the response or an Error.
    ///
    /// # Arguments
    ///
    /// * `uri` - A reference to the Uri to invoke the method on.
    /// * `method` - The name of the method to invoke.
    /// * `args` - Optional msgpack buffer representing the arguments to the method.
    /// * `env` - Optional msgpack buffer representing the environment for the method.
    /// * `resolution_context` - Optional TODO.
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error>;

    /// Returns a list of Uris that this invoker can handle.
    ///
    /// # Arguments
    ///
    /// * `uri` - A reference to the Uri to get the implementations for.
    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error>;

    /// Returns a list of interfaces that this invoker can handle.
    fn get_interfaces(&self) -> Option<InterfaceImplementations>;

    /// Returns the environment for a given Uri.
    ///
    /// # Arguments
    ///
    /// * `uri` - A reference to the Uri to get the environment for.
    fn get_env_by_uri(&self, uri: &Uri) -> Option<Vec<u8>>;
}