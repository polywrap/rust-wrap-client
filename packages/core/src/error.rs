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
  #[error("Failed to create wrapper: `{0}`")]
  WrapperCreateError(String),
  #[error("Failed to invoke wrapper: `{0}`")]
  InvokeError(String),
  #[error("Error loading wrapper: `{0}`")]
  LoadWrapperError(String),
  #[error("WasmWrapper error: `{0}`")]
  WasmWrapperError(String),
  #[error("Failed to resolve wrapper: `{0}`")]
  ResolutionError(String),
  #[error("`{0}`")]
  MsgpackError(String),
  #[error("`{0}`")]
  ManifestError(String),
  #[error("Error reading file: `{0}`")]
  FileReadError(String),
}
