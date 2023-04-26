use std::sync::{Arc, Mutex};

use polywrap_client::core::{wrapper::Wrapper, error::Error, uri::Uri};
use serde_json::Value;

use crate::invoker::FFIInvoker;

pub struct FFIWrapper(Arc<Mutex<Box<dyn Wrapper>>>);

impl FFIWrapper {
  pub fn new(wrapper: Arc<Mutex<Box<dyn Wrapper>>>) -> FFIWrapper {
    FFIWrapper(wrapper)
  }

  pub fn invoke(
    &self,
    invoker: FFIInvoker,
    uri: Arc<Uri>,
    method: &str,
    args: Option<Vec<u8>>,
    env: Option<String>,
  ) -> Result<Vec<u8>, Error> {
    let args = if let Some(args) = &args {
      Some(args.as_slice())
    } else {
        None
    };

    let env = if let Some(env) = env {
        Some(serde_json::from_str::<Value>(&env).unwrap())
    } else {
        None
    };

    self.0.lock().unwrap().invoke(invoker.inner_invoker, uri.as_ref(), method, args, env, None)
  }
}