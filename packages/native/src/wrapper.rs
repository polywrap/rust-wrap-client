use std::sync::{Arc};

use polywrap_client::core::{wrapper::Wrapper, error::Error, uri::Uri};
use serde_json::Value;

use crate::{invoker::FFIInvoker};

pub struct FFIWrapper(pub Arc<dyn Wrapper>);

impl FFIWrapper {
  pub fn new(wrapper: Arc<dyn Wrapper>) -> FFIWrapper {
    FFIWrapper(wrapper)
  }

  pub fn invoke(
    &self,
    invoker: Arc<FFIInvoker>,
    uri: Arc<Uri>,
    method: &str,
    args: Option<Vec<u8>>,
    env: Option<String>,
  ) -> Result<Vec<u8>, Error> {
    let args = args.as_deref();

    let mut _decoded_env = serde_json::Value::Null;
    let env = env.map(|env| {
      _decoded_env = serde_json::from_str::<Value>(&env).unwrap();
      &_decoded_env
    });

    self.0.invoke(invoker.clone(), uri.as_ref(), method, args, env, None)
  }
}