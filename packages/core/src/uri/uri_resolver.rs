use std::future::Future;

use crate::client::Client;
use crate::error::CoreError;

use super::uri::{ Uri };
use super::uri_resolution_context::{ UriResolutionContext, UriPackageOrWrapper };

pub struct TryResolveUriOptions {
  uri: Uri,
  resolution_context: Option<UriResolutionContext>,
}

pub trait UriResolverHandler {
  fn try_resolve_uri(&self, options: Option<&TryResolveUriOptions>) -> dyn Future<Output = Result<UriPackageOrWrapper, CoreError>>;
}

pub trait UriResolver {
  fn try_resolve_uri(&self, uri: &Uri, client: Box<dyn Client>, resolution_context: UriResolutionContext) -> dyn Future<Output = Result<UriPackageOrWrapper, CoreError>>;
}
