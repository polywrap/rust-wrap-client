use std::sync::{Arc, Mutex};

use crate::{invoke::{InvokeOptions, InvokerOptions}, error::CoreError};
pub enum Encoding {
  Base64,
  UTF8
}

pub struct GetFileOptions {
  pub path: String,
  pub encoding: Option<Encoding>,
}

pub trait Wrapper {
  fn invoke(&mut self, options: &InvokeOptions, invoke: Arc<Mutex<dyn FnMut(InvokerOptions) -> Result<Vec<u8>, CoreError> + Send + Sync>>) -> Result<Vec<u8>, CoreError>;
  fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, CoreError>;
}
