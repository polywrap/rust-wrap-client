use crate::client::Client;
use crate::error::CoreError;
use async_trait::async_trait;

use super::uri::{ Uri };
use super::uri_resolution_context::{ UriResolutionContext, UriPackageOrWrapper };

pub struct TryResolveUriOptions {
  pub uri: Uri,
  pub resolution_context: Option<UriResolutionContext>,
}

#[async_trait(?Send)]
pub trait UriResolverHandler {
  async fn try_resolve_uri(&mut self, options: &TryResolveUriOptions) -> Result<UriPackageOrWrapper, CoreError>;
}

#[async_trait]
pub trait UriResolver {
  async fn try_resolve_uri(&self, uri: &Uri, client: Box<&dyn Client>, resolution_context: &UriResolutionContext) -> Result<UriPackageOrWrapper, CoreError>;
}
