use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{wrapper::Wrapper, error::Error, uri_resolution_context::UriResolutionContext, uri::Uri, uri_resolver::UriResolverHandler, env::{Env}, invoke::Invoker};

#[async_trait]
pub trait Loader: UriResolverHandler + Send + Sync {
    async fn load_wrapper(&self, uri: &Uri, resolution_context: Option<&mut UriResolutionContext>,) -> Result<Arc<Mutex<dyn Wrapper>>, Error>;
    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env>;
    fn get_invoker(&self) -> Result<Arc<Mutex<dyn Invoker>>, Error>;
}