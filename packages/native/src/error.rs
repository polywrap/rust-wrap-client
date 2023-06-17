use std::collections::HashMap;

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

impl From<polywrap_client::core::error::Error> for FFIError {
    fn from(value: polywrap_client::core::error::Error) -> Self {
        match value {
            polywrap_client::core::error::Error::UriParseError(err) => {
                FFIError::UriParseError { err }
            }
            polywrap_client::core::error::Error::RedirectsError(err, resolution_stack) => {
                FFIError::RedirectsError {
                    err,
                    resolution_stack,
                }
            }
            polywrap_client::core::error::Error::WrapperError(err) => {
                FFIError::WrapperError { err }
            }
            polywrap_client::core::error::Error::WrapperCreateError(err) => {
                FFIError::WrapperCreateError { err }
            }
            polywrap_client::core::error::Error::InvokeError(uri, method, err) => {
                FFIError::InvokeError { uri, method, err }
            }
            polywrap_client::core::error::Error::LoadWrapperError(uri, err) => {
                FFIError::LoadWrapperError { uri, err }
            }
            polywrap_client::core::error::Error::WasmWrapperError(err) => {
                FFIError::WasmWrapperError { err }
            }
            polywrap_client::core::error::Error::ResolutionError(err) => {
                FFIError::ResolutionError { err }
            }
            polywrap_client::core::error::Error::UriNotFoundError(uri) => {
                FFIError::UriNotFoundError { uri }
            }
            polywrap_client::core::error::Error::MsgpackError(err) => {
                FFIError::MsgpackError { err }
            }
            polywrap_client::core::error::Error::ManifestError(err) => {
                FFIError::ManifestError { err }
            }
            polywrap_client::core::error::Error::FileReadError(err) => {
                FFIError::FileReadError { err }
            }
            polywrap_client::core::error::Error::ResolverError(err) => {
                FFIError::ResolverError { err }
            }
            polywrap_client::core::error::Error::PluginError(err) => FFIError::PluginError { err },
            polywrap_client::core::error::Error::RuntimeError(err) => {
                FFIError::RuntimeError { err }
            }
            polywrap_client::core::error::Error::OtherError(err) => FFIError::OtherError { err },
        }
    }
}

impl From<FFIError> for polywrap_client::core::error::Error {
    fn from(value: FFIError) -> Self {
        match value {
            FFIError::UriParseError { err } => {
                polywrap_client::core::error::Error::UriParseError(err)
            }
            FFIError::RedirectsError {
                err,
                resolution_stack,
            } => polywrap_client::core::error::Error::RedirectsError(err, resolution_stack),
            FFIError::WrapperError { err } => {
                polywrap_client::core::error::Error::WrapperError(err)
            }
            FFIError::WrapperCreateError { err } => {
                polywrap_client::core::error::Error::WrapperCreateError(err)
            }
            FFIError::InvokeError { uri, method, err } => {
                polywrap_client::core::error::Error::InvokeError(uri, method, err)
            }
            FFIError::LoadWrapperError { uri, err } => {
                polywrap_client::core::error::Error::LoadWrapperError(uri, err)
            }
            FFIError::WasmWrapperError { err } => {
                polywrap_client::core::error::Error::WasmWrapperError(err)
            }
            FFIError::ResolutionError { err } => {
                polywrap_client::core::error::Error::ResolutionError(err)
            }
            FFIError::UriNotFoundError { uri } => {
                polywrap_client::core::error::Error::UriNotFoundError(uri)
            }
            FFIError::MsgpackError { err } => {
                polywrap_client::core::error::Error::MsgpackError(err)
            }
            FFIError::ManifestError { err } => {
                polywrap_client::core::error::Error::ManifestError(err)
            }
            FFIError::FileReadError { err } => {
                polywrap_client::core::error::Error::FileReadError(err)
            }
            FFIError::ResolverError { err } => {
                polywrap_client::core::error::Error::ResolverError(err)
            }
            FFIError::PluginError { err } => polywrap_client::core::error::Error::PluginError(err),
            FFIError::RuntimeError { err } => {
                polywrap_client::core::error::Error::RuntimeError(err)
            }
            FFIError::OtherError { err } => polywrap_client::core::error::Error::OtherError(err),
        }
    }
}

impl From<uniffi::UnexpectedUniFFICallbackError> for FFIError {
    fn from(e: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::OtherError { err: e.reason }
    }
}
