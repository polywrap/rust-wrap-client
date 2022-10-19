use std::iter::Map;
use async_trait::async_trait;
use crate::{uri::{uri::Uri, uri_resolution_context::UriResolutionContext}, error::CoreError, wrapper::Wrapper};

pub enum Args {
  Map(Map<String, String>),
  UInt8Array(Box<[u8]>)
}

pub struct InvokeOptions {
  pub uri: Uri,
  pub method: String,
  pub args: Option<Args>,
  pub env: Option<Map<String, String>>,
  pub resolution_context: Option<UriResolutionContext>,
}

pub struct InvokerOptions {
  pub invoke_options: InvokeOptions,
  pub encode_result: bool,
}

pub struct InvocableResult<TData> {
  pub result: Result<TData, CoreError>,
  pub encoded: Option<bool>
}

#[async_trait]
pub trait Invoker {
  fn invoke_wrapper(&self, options: &InvokerOptions, wrapper: Box<dyn Wrapper>) -> Result<String, CoreError>;
  fn invoke(&self, options: InvokerOptions) -> Result<String, CoreError>;
}

#[async_trait]
pub trait Invocable<I: Invoker> {
    fn invoke<TData>(&self, options: &InvokeOptions, invoker: I) -> Result<InvocableResult<TData>, CoreError>;
}