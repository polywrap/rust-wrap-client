use crate::{
    error::Error, uri::Uri, resolvers::uri_resolution_context::UriResolutionContext, wrapper::Wrapper, env::{Env}, interface_implementation::InterfaceImplementations,
};
use std::{sync::Arc};
use futures::lock::Mutex;

pub trait Invoker: Send + Sync {
    fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
    fn get_implementations(&self, uri: Uri) -> Result<Vec<Uri>, Error>;
    fn get_interfaces(&self) -> Option<InterfaceImplementations>;
}
