use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    env::Env, error::Error, invoke::Invoker,
    resolvers::uri_resolution_context::UriResolutionContext,
    resolvers::uri_resolver::UriResolverHandler, uri::Uri, wrapper::Wrapper,
};

#[async_trait]
pub trait Loader: UriResolverHandler + Send + Sync {
    async fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<Mutex<dyn Wrapper>>, Error>;
    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env>;
    fn get_invoker(&self) -> Result<Arc<Mutex<dyn Invoker>>, Error>;
}
