#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("`{0}`")]
  DeserializeError(String),
  #[error("`{0}`")]
  ValidationError(String),
  #[error("`{0}`")]
  JSONError(String)
}