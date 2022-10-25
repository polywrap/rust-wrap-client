use crate::{wrapper::Wrapper, error::Error};
use async_trait::async_trait;

#[async_trait]
pub trait WrapPackage: Send + Sync {
  async fn create_wrapper(&self) -> Result<Box<dyn Wrapper>, Error>;
}