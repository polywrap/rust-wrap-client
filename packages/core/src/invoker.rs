use std::sync::{Mutex, Arc};

use crate::{
    error::Error, uri::Uri, resolution::uri_resolution_context::UriResolutionContext, 
    interface_implementation::InterfaceImplementations,
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
    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error>;
    fn get_interfaces(&self) -> Option<InterfaceImplementations>;
    fn get_env_by_uri(&self, uri: &Uri) -> Option<Vec<u8>>;
}
