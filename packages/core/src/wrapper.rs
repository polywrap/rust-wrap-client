use std::future::Future;

use crate::invoke::{InvokeOptions, Invoker};

pub struct GetFileOptions {
  pub path: String,
  pub encoding: Option<String>,
}

pub trait Wrapper {
  fn invoke<I: Invoker>(&self, options: &InvokeOptions, invoker: I) -> dyn Future<Output = Result<String, String>>;
  fn get_file(&self, options: &GetFileOptions) -> dyn Future<Output = Result<String, String>>;
}
