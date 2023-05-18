use std::{sync::{Arc}, fmt::Debug};

use polywrap_client::core::{wrapper::{Wrapper, GetFileOptions}, error::Error, resolvers::uri_resolution_context::UriResolutionContext, invoker::Invoker, uri::Uri};

use crate::{invoker::FFIInvoker, uri::FFIUri};

pub trait FFIWrapper: Debug + Send + Sync {
  fn invoke(
      &self,
      uri: Arc<FFIUri>,
      method: String,
      args: Option<Vec<u8>>,
      invoker: Arc<FFIInvoker>,
      env: Option<Vec<u8>>,
  ) -> Vec<u8>;
}

#[derive(Debug)]
pub struct ExtWrapper(pub Box<dyn FFIWrapper>);

impl Wrapper for ExtWrapper {
  fn invoke(
    &self,
    invoker: Arc<dyn Invoker>,
    uri: &Uri,
    method: &str,
    args: Option<&[u8]>,
    env: Option<&[u8]>,
    _: Option<&mut UriResolutionContext>,
  ) -> Result<Vec<u8>, Error> {
    let invoker = Arc::new(FFIInvoker::new(invoker));
    let uri = Arc::new(FFIUri::from_string(&uri.to_string()));
    let args = args.map(|args| args.to_vec());
    let env = env.map(|env| env.to_vec());

    Ok(self.0.invoke(uri, method.to_string(), args, invoker, env))
  }

  fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
    unimplemented!("FFI Wrapper does not implement get_file")
  }
}
