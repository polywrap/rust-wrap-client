use std::sync::{Arc, Mutex};

use polywrap_client::core::{wrapper::Wrapper, error::Error, uri::Uri};
use serde_json::Value;

use crate::invoker::FFIInvoker;

pub struct FFIWrapper(pub Arc<Mutex<Box<dyn Wrapper>>>);

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
    let args = args.as_deref();

    let env = env.map(|env| serde_json::from_str::<Value>(&env).unwrap());

    self.0.lock().unwrap().invoke(invoker.inner_invoker, uri.as_ref(), method, args, env, None)
  }
}