use std::sync::{Arc, Mutex};

use polywrap_core::{
    error::Error, interface_implementation::InterfaceImplementations, invoker::Invoker,
    resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
};

pub struct Subinvoker {
    resolution_context: Arc<Mutex<UriResolutionContext>>,
    invoker: Arc<dyn Invoker>,
}

impl Subinvoker {
    pub fn new(
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Self {
        Self {
            invoker,
            resolution_context,
        }
    }
}

impl Invoker for Subinvoker {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        _: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        let context = self.resolution_context.clone();
        self.invoker
            .invoke_raw(uri, method, args, env, Some(context))
    }
    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
        self.invoker.get_implementations(uri)
    }
    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        self.invoker.get_interfaces()
    }
    fn get_env_by_uri(&self, uri: &Uri) -> Option<Vec<u8>> {
        self.invoker.get_env_by_uri(uri)
    }
    fn get_file(
        &self,
        uri: &Uri,
        path: String,
        _: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Option<Vec<u8>>, Error> {
        self.invoker.get_file(uri, path, None)
    }
}
