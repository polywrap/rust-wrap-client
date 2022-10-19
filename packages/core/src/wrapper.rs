use std::future::Future;

use crate::invoke::{InvokeOptions, Invoker};

pub struct GetFileOptions {
  pub path: String,
  pub encoding: Option<String>,
}

pub trait Wrapper {
  fn invoke(&self, options: &InvokeOptions, invoker: Box<dyn Invoker>) -> dyn Future<Output = Result<String, String>>;
  fn get_file(&self, options: &GetFileOptions) -> dyn Future<Output = Result<String, String>>;
}
