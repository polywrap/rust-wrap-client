use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrapperError {
    #[error("`{0}`")]
    ModuleReadError(String),
    #[error("`{0}`")]
    LoadWrapperError(String),
    #[error("`{0}`")]
    FileReadError(#[from] std::io::Error),
    #[error("Invocation error: `{0}`")]
    InvokeError(String),
    #[error("`{0}`")]
    DecodeError(String),
    #[error("`{0}`")]
    WasmRuntimeError(String),
    #[error("`{0}`")]
    ExportError(String),
    #[error("`{0}`")]
    CompilationError(#[from] wasmer::CompileError),
    #[error("`{0}`")]
    MemoryError(#[from] wasmer::MemoryError),
    #[error("`{0}`")]
    ModuleSerializeError(#[from] wasmer::SerializeError),
    #[error("`{0}`")]
    ModuleDeserializeError(#[from] wasmer::DeserializeError),
}

impl From<WrapperError> for polywrap_core::error::Error {
    fn from(error: WrapperError) -> Self {
        polywrap_core::error::Error::WrapperError(error.to_string())
    }
}
