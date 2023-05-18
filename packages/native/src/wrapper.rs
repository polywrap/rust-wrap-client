use std::{sync::{Arc}, fmt::Debug};

use polywrap_client::core::{wrapper::{Wrapper, GetFileOptions}, error::Error, resolvers::uri_resolution_context::UriResolutionContext, invoker::Invoker, uri::Uri};

use crate::{invoker::FFIInvoker, uri::FFIUri};

pub trait FFIWrapper: Debug + Send + Sync {
  fn ffi_invoke(
      &self,
      invoker: Arc<FFIInvoker>,
      uri: Arc<FFIUri>,
      method: &str,
      args: Option<Vec<u8>>,
      env: Option<Vec<u8>>,
  ) -> Result<Vec<u8>, Error>;
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

    self.0.ffi_invoke(invoker, uri, method, args, env)
  }

  fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
    unimplemented!("FFI Wrapper does not implement get_file")
  }
}
