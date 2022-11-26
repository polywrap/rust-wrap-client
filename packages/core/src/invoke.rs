use crate::{
    error::Error, uri::Uri, resolvers::uri_resolution_context::UriResolutionContext, wrapper::Wrapper, env::Env, interface_implementation::InterfaceImplementations,
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
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
    async fn invoke(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
    async fn get_implementations(&self, uri: Uri) -> Result<Vec<Uri>, Error>;
    fn get_interfaces(&self) -> Option<InterfaceImplementations>;
}
