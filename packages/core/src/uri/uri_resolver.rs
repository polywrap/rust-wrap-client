use std::future::Future;
use std::pin::Pin;

use crate::client::Client;
use crate::error::CoreError;

use super::uri::{ Uri };
use super::uri_resolution_context::{ UriResolutionContext, UriPackageOrWrapper };

pub struct TryResolveUriOptions {
  pub uri: Uri,
  pub resolution_context: Option<UriResolutionContext>,
}

pub trait UriResolverHandler {
  fn try_resolve_uri(&mut self, options: &TryResolveUriOptions) -> Pin<Box<dyn Future<Output = Result<UriPackageOrWrapper, CoreError>>>>;
}

pub trait UriResolver {
  fn try_resolve_uri(&self, uri: &Uri, client: Box<&dyn Client>, resolution_context: &UriResolutionContext) -> Pin<Box<dyn Future<Output = Result<UriPackageOrWrapper, CoreError>>>>;
}
