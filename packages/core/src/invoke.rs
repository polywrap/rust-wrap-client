use async_trait::async_trait;
use crate::{uri::Uri, uri_resolution_context::UriResolutionContext, error::Error, wrapper::Wrapper, env::Env};

pub enum InvokeArgs {
  Msgpack(polywrap_msgpack::Value),
  UIntArray(Vec<u8>)
}

pub struct InvokeOptions<'a> {
  pub uri: &'a Uri,
  pub method: &'a str,
  pub args: Option<&'a InvokeArgs>,
  pub env: Option<&'a Env>,
  pub resolution_context: Option<&'a UriResolutionContext>,
}

#[async_trait]
pub trait Invoker: Send + Sync {
  async fn invoke_wrapper(&self, options: &InvokeOptions, wrapper: Box<dyn Wrapper>) -> Result<Vec<u8>, Error>;
  async fn invoke(&self, options: &InvokeOptions) -> Result<Vec<u8>, Error>;
}
