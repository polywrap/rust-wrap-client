use polywrap_core::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum PluginError {
  #[error("`{0}`")]
  ModuleError(String),

  #[error("PluginWrapper: invocation exception encountered.\nuri: {uri:?}\nmethod: {method:?}\nargs: {args:?}\nexception: {exception:?}")]
  InvocationError {
    uri: String,
    method: String,
    args: String,
    exception: String
  },

  #[error("Subinvocation exception encountered.\nuri: {uri:?}\nmethod: {method:?}\nargs: {args:?}\nexception: {exception:?}")]
  SubinvocationError {
    uri: String,
    method: String,
    args: String,
    exception: String
  },

  #[error("Method '`{0}`' not found")]
  MethodNotFoundError(String),

  #[error(transparent)]
  JSONError(#[from] serde_json::error::Error),

  #[error(transparent)]
  MsgpackEncodeError(#[from] rmp_serde::encode::Error),

  #[error(transparent)]
  MsgpackDecodeError(#[from] rmp_serde::decode::Error),
}

impl From<PluginError> for Error {
    fn from(e: PluginError) -> Self {
      Error::PluginError(e.to_string())
    }
}