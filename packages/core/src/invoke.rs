use std::iter::Map;
use std::future::Future;

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

pub trait Invoker {
  fn invoke_wrapper<TData, W: Wrapper>(&self, options: &InvokerOptions, wrapper: W) -> dyn Future<Output = Result<TData, CoreError>>;
  fn invoke<TData>(&self, options: InvokerOptions) -> dyn Future<Output = Result<TData, CoreError>>;
}

pub trait Invocable<I: Invoker> {
    fn invoke<TData>(&self, options: &InvokeOptions, invoker: I) -> dyn Future<Output = Result<InvocableResult<TData>, CoreError>>;
}