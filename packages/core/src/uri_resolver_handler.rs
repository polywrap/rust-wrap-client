use crate::{resolvers::uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}, error::Error, uri::Uri};

pub trait UriResolverHandler {
  fn try_resolve_uri(
      &self,
      uri: &Uri,
      resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<UriPackageOrWrapper, Error>;
}
