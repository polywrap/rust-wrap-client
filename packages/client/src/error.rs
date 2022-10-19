use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
  #[error("`{0}`")]
  LoadWrapperError(String),
}
