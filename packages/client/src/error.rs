use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("`{0}`")]
  LoadWrapperError(String),
}

impl From<Error> for polywrap_core::error::Error {
  fn from(error: Error) -> Self {
    let a = error.to_string();
    Self::ClientError(error.to_string())
  }
}