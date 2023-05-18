use crate::{error::Error, uri::Uri, resolution::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext}};

pub trait UriResolverHandler {
  fn try_resolve_uri(
      &self,
      uri: &Uri,
      resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<UriPackageOrWrapper, Error>;
}
