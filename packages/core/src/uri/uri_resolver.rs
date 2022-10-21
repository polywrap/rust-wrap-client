use crate::client::Client;
use crate::error::CoreError;
use async_trait::async_trait;

use super::uri::{ Uri };
use super::uri_resolution_context::{ UriResolutionContext, UriPackageOrWrapper };

#[async_trait(?Send)]
pub trait UriResolverHandler {
  async fn try_resolve_uri(&self, uri: &Uri, resolution_context: Option<&UriResolutionContext>) -> Result<UriPackageOrWrapper, CoreError>;
}

#[async_trait]
pub trait UriResolver: Send + Sync {
  async fn try_resolve_uri(&self, uri: &Uri, client: &dyn Client, resolution_context: &UriResolutionContext) -> Result<UriPackageOrWrapper, CoreError>;
}
