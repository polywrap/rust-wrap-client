use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
  #[error("Error parsing URI: `{0}`")]
  UriParseError(String),
  #[error("Error getting file: `{0}`")]
  GetFileError(String),
  #[error("`{0}`\nResolution Stack: `{1:#?}`")]
  RedirectsError(String, HashMap<String, String>),
  #[error("`{0}`")]
  WrapperError(String),
  #[error("`{0}`")]
  InvokeError(String),
}
