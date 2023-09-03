use std::sync::{Arc, Mutex};

use crate::{
    error::Error, interface_implementation::InterfaceImplementations,
    resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
};
pub trait Invoker: Send + Sync {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error>;
    fn get_file(
        &self, 
        uri: &Uri, 
        path: String,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Option<Vec<u8>>, Error>;
    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error>;
    fn get_interfaces(&self) -> Option<InterfaceImplementations>;
    fn get_env_by_uri(&self, uri: &Uri) -> Option<Vec<u8>>;
}
