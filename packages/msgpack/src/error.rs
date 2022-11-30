#[derive(thiserror::Error, Debug)]
pub enum MsgpackError {
  #[error("`{0}`")]
  EncodeError(String),

  #[error("`{0}`")]
  DecodeError(String),
}

impl From<rmp_serde::encode::Error> for MsgpackError {
    fn from(e: rmp_serde::encode::Error) -> Self {
        MsgpackError::EncodeError(e.to_string())
    }
}

impl From<rmp_serde::decode::Error> for MsgpackError {
  fn from(e: rmp_serde::decode::Error) -> Self {
      MsgpackError::DecodeError(e.to_string())
  }
}

impl From<rmpv::encode::Error> for MsgpackError {
  fn from(e: rmpv::encode::Error) -> Self {
      MsgpackError::EncodeError(e.to_string())
  }
}