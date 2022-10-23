use std::{iter::Map, sync::Arc};
use async_trait::async_trait;
use crate::{uri::Uri, uri_resolution_context::UriResolutionContext, error::CoreError, client::Client, wrapper::Wrapper};

pub struct InvokeOptions<'a> {
  pub uri: &'a Uri,
  pub method: &'a str,
  pub args: Option<&'a Vec<u8>>,
  pub env: Option<&'a Map<String, String>>,
  pub resolution_context: Option<&'a UriResolutionContext>,
}

pub struct InvokerOptions<'a> {
  pub invoke_options: InvokeOptions<'a>,
  pub encode_result: bool,
}

pub struct InvocableResult<TData> {
  pub result: Result<TData, CoreError>,
  pub encoded: Option<bool>
}

#[async_trait(?Send)]
pub trait Invoker: Send + Sync {
  async fn invoke_wrapper(&self, options: &InvokerOptions, wrapper: Arc<dyn Wrapper>) -> Result<Vec<u8>, CoreError>;
  async fn invoke(&self, options: &InvokerOptions) -> Result<Vec<u8>, CoreError>;
}

#[async_trait]
pub trait Invocable<C: Client> {
    fn invoke<TData>(&self, options: InvokeOptions, invoker: C) -> Result<InvocableResult<TData>, CoreError>;
}
