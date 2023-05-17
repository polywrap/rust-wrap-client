use std::sync::{Arc};

use polywrap_client::core::{wrapper::Wrapper, error::Error};

use crate::{invoker::FFIInvoker, uri::FFIUri};

pub struct FFIWrapper(pub Arc<dyn Wrapper>);

impl FFIWrapper {
  pub fn new(wrapper: Arc<dyn Wrapper>) -> FFIWrapper {
    FFIWrapper(wrapper)
  }

  pub fn invoke(
    &self,
    invoker: Arc<FFIInvoker>,
    uri: Arc<FFIUri>,
    method: &str,
    args: Option<Vec<u8>>,
    env: Option<Vec<u8>>,
  ) -> Result<Vec<u8>, Error> {
    let args = args.as_deref();

    self.0.invoke(invoker.clone(), &uri.0, method, args, env.map(|e| e.as_slice()), None)
  }
}