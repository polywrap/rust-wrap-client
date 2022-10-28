use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
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
  #[error("Client error: `{0}`")]
  ClientError(String),
  #[error("WasmWrapper error: `{0}`")]
  WasmWrapperError(String),
  #[error("`{0}`")]
  ResolutionError(String),
  #[error("`{0}`")]
  MsgpackError(String),
}
