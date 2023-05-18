use std::{sync::Arc, fmt::Debug, any::Any};

use crate::{error::Error, invoker::{Invoker}, uri::Uri, resolvers::uri_resolution_context::UriResolutionContext};
pub enum Encoding {
    Base64,
    UTF8,
}

pub struct GetFileOptions {
    pub path: String,
    pub encoding: Option<Encoding>,
}

pub trait Wrapper: Send + Sync + Debug + Any {
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
        abort_handler: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<Vec<u8>, Error>;
    fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error>;
}
