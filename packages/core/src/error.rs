use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
  #[error("Error parsing URI: `{0}`")]
  UriParseError(String),
  #[error("`{0}`\nResolution Stack: `{1:#?}`")]
  RedirectsError(String, HashMap<String, String>)
}