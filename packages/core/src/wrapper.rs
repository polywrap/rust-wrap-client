use crate::{invoke::{InvokeOptions, Invoker}, error::CoreError};
use async_trait::async_trait;

pub struct GetFileOptions {
  pub path: String,
  pub encoding: Option<String>,
}

#[async_trait]
pub trait Wrapper {
  async fn invoke(&self, options: &InvokeOptions, invoker: Box<dyn Invoker>) -> Result<String, CoreError>;
  async fn get_file(&self, options: &GetFileOptions) -> Result<String, CoreError>;
}
