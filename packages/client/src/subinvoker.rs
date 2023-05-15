use std::sync::{RwLock, Arc};

use polywrap_core::{
  resolvers::uri_resolution_context::{UriResolutionContext, UriResolutionStep}, 
  invoker::Invoker, env::Env, error::Error, uri::Uri, interface_implementation::InterfaceImplementations
};

pub struct Subinvoker {
  pub invoke_context: RwLock<UriResolutionContext>,
  invoker: Arc<dyn Invoker>,
}

impl Subinvoker {
  pub fn new(
      invoker: Arc<dyn Invoker>,
      invoke_context: UriResolutionContext,
  ) -> Self {
      Self {
          invoker,
          invoke_context: RwLock::new(invoke_context),
      }
  }

  pub fn get_history(&self) -> Vec<UriResolutionStep> {
      self.invoke_context.read().unwrap().get_history().clone()
  }
}

impl Invoker for Subinvoker {
  fn invoke_raw(
      &self,
      uri: &Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&Env>,
      _: Option<&mut UriResolutionContext>,
  ) -> Result<Vec<u8>, Error> {
      let mut context = self.invoke_context.write().unwrap();
      self.invoker.invoke_raw(uri, method, args, env, Some(&mut context))
  }
  fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
      self.invoker.get_implementations(uri)
  }
  fn get_interfaces(&self) -> Option<InterfaceImplementations> {
      self.invoker.get_interfaces()
  }
}
