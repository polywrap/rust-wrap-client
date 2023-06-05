use std::collections::HashMap;

#[derive(thiserror::Error, Debug, Clone)]
pub enum FFIError {
  #[error("Error parsing URI: `{err}`")]
  UriParseError{ err: String },
  #[error("`{err}`\nResolution Stack: `{resolution_stack:#?}`")]
  RedirectsError{ err: String, resolution_stack: HashMap<String, String> },
  #[error("`{err}`")]
  WrapperError{ err: String },
  #[error("Failed to create wrapper: `{err}`")]
  WrapperCreateError{ err: String },
  #[error("Failed to invoke wrapper, uri: `{uri}`, method: `{method}`: `{err}`")]
  InvokeError{ uri: String, method: String, err: String },
  #[error("Error loading wrapper: `{err}`")]
  LoadWrapperError{ err: String },
  #[error("WasmWrapper error: `{err}`")]
  WasmWrapperError{ err: String },
  #[error("Failed to resolve wrapper: `{err}`")]
  ResolutionError{ err: String },
  #[error("`{err}`")]
  MsgpackError{ err: String },
  #[error("`{err}`")]
  ManifestError{ err: String },
  #[error("Error reading file: `{err}`")]
  FileReadError{ err: String },
  #[error("`{err}`")]
  ResolverError{ err: String },
  #[error("`{err}`")]
  PluginError{ err: String },
  #[error("`{err}`")]
  RuntimeError{ err: String },
  #[error("`{err}`")]
  UnexpectedError{ err: String }
}

impl From<polywrap_client::core::error::Error> for FFIError {
    fn from(value: polywrap_client::core::error::Error) -> Self {
        todo!()
    }
}

impl From<FFIError> for polywrap_client::core::error::Error {
    fn from(value: FFIError) -> Self {
        todo!()
    }
}

impl From<uniffi::UnexpectedUniFFICallbackError> for FFIError {
  fn from(e: uniffi::UnexpectedUniFFICallbackError) -> Self {
      Self::UnexpectedError { err: e.reason }
  }
}