use std::sync::{Arc};

use async_trait::async_trait;

use crate::{invoke::{InvokeOptions, Invoker}, error::Error};
pub enum Encoding {
  Base64,
  UTF8
}

pub struct GetFileOptions {
  pub path: String,
  pub encoding: Option<Encoding>,
}

#[async_trait]
pub trait Wrapper: Send + Sync {
  async fn invoke(&self, options: &InvokeOptions, invoker: Arc<dyn Invoker>) -> Result<Vec<u8>, Error>;
  // fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error>;
}
