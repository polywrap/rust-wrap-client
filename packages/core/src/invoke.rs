use std::{iter::Map};
use async_trait::async_trait;
use crate::{uri::Uri, uri_resolution_context::UriResolutionContext, error::Error, wrapper::Wrapper};

pub enum InvokeArgs {
  JSON(serde_json::Value),
  UIntArray(Vec<u8>)
}

pub struct InvokeOptions<'a> {
  pub uri: &'a Uri,
  pub method: &'a str,
  pub args: Option<&'a InvokeArgs>,
  pub env: Option<&'a Map<String, String>>,
  pub resolution_context: Option<&'a UriResolutionContext>,
}

#[async_trait(?Send)]
pub trait Invoker: Send + Sync {
  async fn invoke_wrapper(&self, options: &InvokeOptions, wrapper: Box<dyn Wrapper>) -> Result<Vec<u8>, Error>;
  async fn invoke(&self, options: &InvokeOptions) -> Result<Vec<u8>, Error>;
}
