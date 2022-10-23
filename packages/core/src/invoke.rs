use std::{iter::Map, sync::{Arc}};
use async_trait::async_trait;
use crate::{uri::Uri, uri_resolution_context::UriResolutionContext, error::CoreError, wrapper::Wrapper};

pub struct InvokeOptions<'a> {
  pub uri: &'a Uri,
  pub method: &'a str,
  pub args: Option<&'a Vec<u8>>,
  pub env: Option<&'a Map<String, String>>,
  pub resolution_context: Option<&'a UriResolutionContext>,
}

#[async_trait(?Send)]
pub trait Invoker: Send + Sync {
  async fn invoke_wrapper(&self, options: &InvokeOptions, wrapper: Arc<dyn Wrapper>) -> Result<Vec<u8>, CoreError>;
  async fn invoke(&self, options: &InvokeOptions) -> Result<Vec<u8>, CoreError>;
}

