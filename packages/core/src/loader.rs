use async_trait::async_trait;

use crate::{wrapper::Wrapper, error::Error, uri_resolution_context::UriResolutionContext, uri::Uri, uri_resolver::UriResolverHandler};

#[async_trait(?Send)]
pub trait Loader: UriResolverHandler {
    async fn load_wrapper(&self, uri: &Uri, resolution_context: Option<&UriResolutionContext>,) -> Result<Box<dyn Wrapper>, Error>;
}