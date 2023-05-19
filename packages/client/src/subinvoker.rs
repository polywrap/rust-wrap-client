use std::sync::{Mutex, Arc};

use polywrap_core::{
    resolution::uri_resolution_context::UriResolutionContext, 
    invoker::Invoker, env::Env, error::Error, uri::Uri, interface_implementation::InterfaceImplementations
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
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut context = self.resolution_context.lock().unwrap();
        self.invoker.invoke_raw(uri, method, args, env, Some(&mut context))
    }
    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
        self.invoker.get_implementations(uri)
    }
    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        self.invoker.get_interfaces()
    }
    fn get_env_by_uri(&self, uri: &Uri) -> Option<&[u8]> {
        self.invoker.get_env_by_uri(uri)
    }
}
