use std::collections::HashMap;

use polywrap_client::core::error::Error;

#[derive(thiserror::Error, Debug, Clone)]
pub enum FFIError {
    #[error("Error parsing URI: `{err}`")]
    UriParseError { err: String },
    #[error("`{err}`\nResolution Stack: `{resolution_stack:#?}`")]
    RedirectsError {
        err: String,
        resolution_stack: HashMap<String, String>,
    },
    #[error("`{err}`")]
    WrapperError { err: String },
    #[error("Failed to create wrapper: `{err}`")]
    WrapperCreateError { err: String },
    #[error("Failed to invoke wrapper, uri: `{uri}`, method: `{method}`: `{err}`")]
    InvokeError {
        uri: String,
        method: String,
        err: String,
    },
    #[error("Error loading wrapper: `{err}`")]
    LoadWrapperError { uri: String, err: String },
    #[error("WasmWrapper error: `{err}`")]
    WasmWrapperError { err: String },
    #[error("Failed to resolve wrapper: `{err}`")]
    ResolutionError { err: String },
    #[error("URI not found: `{uri}`")]
    UriNotFoundError { uri: String },
    #[error("`{err}`")]
    MsgpackError { err: String },
    #[error("`{err}`")]
    ManifestError { err: String },
    #[error("Error reading file: `{err}`")]
    FileReadError { err: String },
    #[error("`{err}`")]
    ResolverError { err: String },
    #[error("`{err}`")]
    PluginError { err: String },
    #[error("`{err}`")]
    RuntimeError { err: String },
    #[error("`{err}`")]
    OtherError { err: String },
}

impl From<polywrap_wasm::error::WrapperError> for FFIError {
    fn from(err: polywrap_wasm::error::WrapperError) -> Self {
      FFIError::WrapperError { err: err.to_string() }
    }
}

impl From<Error> for FFIError {
    fn from(value: Error) -> Self {
        match value {
            Error::UriParseError(err) => FFIError::UriParseError {
                err: err.to_string(),
            },
            Error::RedirectsError(err, resolution_stack) => {
                FFIError::RedirectsError {
                    err,
                    resolution_stack,
                }
            }
            Error::WrapperError(err) => {
                FFIError::WrapperError { err }
            }
            Error::WrapperCreateError(err) => {
                FFIError::WrapperCreateError { err }
            }
            Error::InvokeError(uri, method, err) => {
                FFIError::InvokeError { uri, method, err }
            }
            Error::LoadWrapperError(uri, err) => {
                FFIError::LoadWrapperError { uri, err }
            }
            Error::WasmWrapperError(err) => {
                FFIError::WasmWrapperError { err }
            }
            Error::ResolutionError(err) => {
                FFIError::ResolutionError { err }
            }
            Error::UriNotFoundError(uri) => {
                FFIError::UriNotFoundError { uri }
            }
            Error::MsgpackError(err) => {
                // let error
                FFIError::MsgpackError {
                    err: err.to_string(),
                }
            }
            Error::ManifestError(err) => {
                FFIError::ManifestError { err }
            }
            Error::FileReadError(err) => {
                FFIError::FileReadError { err }
            }
            Error::ResolverError(err) => {
                FFIError::ResolverError { err }
            }
            Error::PluginError(err) => FFIError::PluginError { err },
            Error::RuntimeError(err) => {
                FFIError::RuntimeError { err }
            }
            Error::OtherError(err) => FFIError::OtherError { err },
        }
    }
}

impl From<FFIError> for Error {
    fn from(value: FFIError) -> Self {
        match value {
            FFIError::UriParseError { err } => Error::UriParseError(
                polywrap_client::core::uri::ParseError(err),
            ),
            FFIError::RedirectsError {
                err,
                resolution_stack,
            } => Error::RedirectsError(err, resolution_stack),
            FFIError::WrapperError { err } => {
                Error::WrapperError(err)
            }
            FFIError::WrapperCreateError { err } => {
                Error::WrapperCreateError(err)
            }
            FFIError::InvokeError { uri, method, err } => {
                Error::InvokeError(uri, method, err)
            }
            FFIError::LoadWrapperError { uri, err } => {
                Error::LoadWrapperError(uri, err)
            }
            FFIError::WasmWrapperError { err } => {
                Error::WasmWrapperError(err)
            }
            FFIError::ResolutionError { err } => {
                Error::ResolutionError(err)
            }
            FFIError::UriNotFoundError { uri } => {
                Error::UriNotFoundError(uri)
            }
            FFIError::MsgpackError { err } => {
                let msgpack = polywrap_msgpack_serde::Error::Message(err);
                Error::MsgpackError(msgpack)
            }
            FFIError::ManifestError { err } => {
                Error::ManifestError(err)
            }
            FFIError::FileReadError { err } => {
                Error::FileReadError(err)
            }
            FFIError::ResolverError { err } => {
                Error::ResolverError(err)
            }
            FFIError::PluginError { err } => Error::PluginError(err),
            FFIError::RuntimeError { err } => {
                Error::RuntimeError(err)
            }
            FFIError::OtherError { err } => Error::OtherError(err),
        }
    }
}

impl From<uniffi::UnexpectedUniFFICallbackError> for FFIError {
    fn from(e: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::OtherError { err: e.reason }
    }
}
