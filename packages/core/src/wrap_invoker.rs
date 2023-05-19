use crate::{resolvers::uri_resolution_context::UriResolutionContext, error::Error, wrapper::Wrapper, uri::Uri};

pub trait WrapInvoker: Send + Sync {
  fn invoke_wrapper_raw(
      &self,
      wrapper: &dyn Wrapper,
      uri: &Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&[u8]>,
      resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<Vec<u8>, Error>;
}
