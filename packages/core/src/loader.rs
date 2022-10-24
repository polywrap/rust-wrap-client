use std::sync::Arc;

use async_trait::async_trait;

use crate::{wrapper::Wrapper, error::CoreError, uri_resolution_context::UriResolutionContext, uri::Uri, uri_resolver::UriResolverHandler};

#[async_trait(?Send)]
pub trait Loader: UriResolverHandler {
    async fn load_wrapper(&self, uri: &Uri, resolution_context: Option<&UriResolutionContext>,) -> Result<Arc<dyn Wrapper>, CoreError>;
}