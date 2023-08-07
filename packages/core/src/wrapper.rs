use std::{any::Any, fmt::Debug, sync::Arc};

use crate::{error::Error, invoker::Invoker};

/// The `Encoding` enum is used to specify the type of encoding for a file.
/// It currently supports Base64 and UTF8 encoding.
pub enum Encoding {
    Base64,
    UTF8,
}

/// Struct used to specify the options when getting a file.
/// It contains a `path` field for the file path and an `encoding` field for the file encoding.
pub struct GetFileOptions {
    /// The path of the file to get.
    pub path: String,
    /// The encoding of the file. This is an optional field.
    pub encoding: Option<Encoding>,
}

/// Common interface for objects that can be invoked and can get files.
/// It requires the implementing type to be Send, Sync, Debug, and Any.
pub trait Wrapper: Send + Sync + Debug + Any {
    /// The `invoke` method is used to invoke the object with a method, arguments, environment, and invoker.
    /// It returns a Result containing a msgpack buffer on success, or an Error on failure.
    ///
    /// # Arguments
    ///
    /// * `method` - The name of the method to invoke.
    /// * `args` - Optional msgpack buffer representing the arguments to the method.
    /// * `env` - Optional msgpack buffer representing the environment for the method.
    /// * `invoker` - `Invoker` to invoke this wrapper with.
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, Error>;

    /// The `get_file` method is used to get a file with the specified options.
    /// It returns a Result containing a byte vector on success, or an Error on failure.
    fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error>;
}
