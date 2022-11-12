use crate::{
    error::Error, uri::Uri, uri_resolution_context::UriResolutionContext, wrapper::Wrapper,
};
use async_trait::async_trait;
use std::{sync::Arc};
use tokio::sync::Mutex;

pub enum InvokeArgs {
    Msgpack(polywrap_msgpack::Value),
    UIntArray(Vec<u8>),
}

#[async_trait]
pub trait Invoker: Send + Sync {
    async fn invoke_wrapper(
        &self,
        wrapper: Arc<Mutex<Box<dyn Wrapper>>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
    async fn invoke(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
}
