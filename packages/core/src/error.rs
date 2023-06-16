use std::collections::HashMap;

use polywrap_msgpack::error::MsgpackError;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
  #[error("Error parsing URI: `{0}`")]
  UriParseError(String),
  #[error("`{0}`\nResolution Stack: `{1:#?}`")]
  RedirectsError(String, HashMap<String, String>),
  #[error("`{0}`")]
  WrapperError(String),
  #[error("Failed to create wrapper: `{0}`")]
  WrapperCreateError(String),
  #[error("Failed to invoke wrapper, uri: `{0}`, method: `{1}`: `{2}`")]
  InvokeError(String, String, String),
  #[error("Error loading wrapper, uri: {0}: `{1}`")]
  LoadWrapperError(String, String),
  #[error("WasmWrapper error: `{0}`")]
  WasmWrapperError(String),
  #[error("Failed to resolve wrapper: `{0}`")]
  ResolutionError(String),
  #[error("URI not found: `{0}`")]
  UriNotFoundError(String),
  #[error("`{0}`")]
  MsgpackError(String),
  #[error("`{0}`")]
  ManifestError(String),
  #[error("Error reading file: `{0}`")]
  FileReadError(String),
  #[error("`{0}`")]
  ResolverError(String),
  #[error("`{0}`")]
  PluginError(String),
  #[error("`{0}`")]
  RuntimeError(String),
  #[error("`{0}`")]
  OtherError(String),
}

impl From<MsgpackError> for Error {
  fn from(e: MsgpackError) -> Self {
    Error::MsgpackError(e.to_string())
  }
}