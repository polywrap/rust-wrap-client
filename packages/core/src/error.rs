use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
  #[error("Error parsing URI: `{0}`")]
  UriParseError(String),
}
