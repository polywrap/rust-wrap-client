use crate::{resolution::uri_resolution_context::UriResolutionContext, error::Error, wrapper::Wrapper, uri::Uri, env::Env};

pub trait WrapInvoker: Send + Sync {
  fn invoke_wrapper_raw(
      &self,
      wrapper: &dyn Wrapper,
      uri: &Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&Env>,
      resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<Vec<u8>, Error>;
}
