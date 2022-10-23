use std::sync::{Arc, Mutex};

use crate::{invoke::{InvokeOptions, Invoker}, error::CoreError};
pub enum Encoding {
  Base64,
  UTF8
}

pub struct GetFileOptions {
  pub path: String,
  pub encoding: Option<Encoding>,
}

pub trait Wrapper: Send + Sync {
  fn invoke(&self, options: &InvokeOptions, invoker: Arc<Mutex<dyn Invoker>>) -> Result<Vec<u8>, CoreError>;
  fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, CoreError>;
}
