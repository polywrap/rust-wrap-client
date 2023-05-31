use polywrap_core::error::Error;
use polywrap_msgpack::error::MsgpackError;

#[derive(thiserror::Error, Debug)]
pub enum PluginError {
  #[error("PluginWrapper: invocation exception encountered.\nexception: {exception:?}")]
  InvocationError {
    exception: String
  },

  #[error("Method '`{0}`' not found")]
  MethodNotFoundError(String),

  #[error(transparent)]
  JSONError(#[from] serde_json::error::Error),

  #[error("`{0}`")]
  MsgpackError(String),
}

impl From<PluginError> for Error {
    fn from(e: PluginError) -> Self {
      Error::PluginError(e.to_string())
    }
}

impl From<MsgpackError> for PluginError {
  fn from(e: MsgpackError) -> Self {
    PluginError::MsgpackError(e.to_string())
  }
}