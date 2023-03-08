#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("`{0}`")]
  DeserializeError(String),
  #[error("`{0}`")]
  ValidationError(String),
  #[error("Serde JSON error: `{0}`")]
  JSONError(String),
  #[error("From JSON conversion error: `{0}`")]
  FromJSONError(String),
  #[error("JSONSchema error: `{0}`")]
  JSONSchemaError(String),
  #[error("Msgpack decode error: `{0}`")]
  MsgpackDecodeError(String),
  #[error("Semver error: `{0}`")]
  SemverError(String),
}

impl From<serde_json::Error> for Error {
  fn from(error: serde_json::Error) -> Self {
    Error::JSONError(error.to_string())
  }
}

impl From<jsonschema::ValidationError<'_>> for Error {
  fn from(error: jsonschema::ValidationError) -> Self {
    Error::JSONSchemaError(error.to_string())
  }
}

impl From<polywrap_msgpack::rmp_serde::decode::Error> for Error {
  fn from(error: polywrap_msgpack::rmp_serde::decode::Error) -> Self {
    Error::MsgpackDecodeError(error.to_string())
  }
}

impl From <semver::Error> for Error {
  fn from(error: semver::Error) -> Self {
    Error::SemverError(error.to_string())
  }
}