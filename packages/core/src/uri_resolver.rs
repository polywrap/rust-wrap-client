use crate::error::Error;
use crate::loader::Loader;
use async_trait::async_trait;

use super::uri::Uri;
use super::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext};

#[async_trait]
pub trait UriResolverHandler {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error>;
}

#[async_trait]
pub trait UriResolver: Send + Sync {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        client: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error>;
}
