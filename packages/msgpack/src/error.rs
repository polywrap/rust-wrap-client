pub use polywrap_serde::error::EncodeError;
pub use polywrap_serde::error::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum MsgpackError {
    #[error(transparent)]
    EncodeError(#[from] EncodeError),

    #[error(transparent)]
    DecodeError(#[from] DecodeError)
}

// impl From<rmp_serde::encode::Error> for MsgpackError {
//     fn from(e: Error) -> Self {
//         MsgpackError::EncodeError(e.to_string())
//     }
// }

// impl From<rmp_serde::decode::Error> for MsgpackError {
//     fn from(e: rmp_serde::decode::Error) -> Self {
//         MsgpackError::DecodeError(e.to_string())
//     }
// }

// impl From<rmpv::encode::Error> for MsgpackError {
//     fn from(e: rmpv::encode::Error) -> Self {
//         MsgpackError::EncodeError(e.to_string())
//     }
// }
