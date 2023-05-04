use core::fmt;
use polywrap_core::{
    error::Error,
    uri::Uri,
    resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    resolvers::uri_resolver::UriResolver, client::Client,
};

pub struct BaseResolver {
  fs_resolver: Box<dyn UriResolver>,
  static_resolver: Box<dyn UriResolver>
}

impl BaseResolver {
  pub fn new(fs_resolver: Box<dyn UriResolver>, static_resolver: Box<dyn UriResolver>) -> Self {
    Self {
      fs_resolver,
      static_resolver
    }
  }
}

impl UriResolver for BaseResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        client: &dyn Client,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let redirected_uri = self.static_resolver.try_resolve_uri(uri, client, resolution_context)?;

        if let UriPackageOrWrapper::Uri(redirected_uri) = redirected_uri {
          self.fs_resolver.try_resolve_uri(&redirected_uri, client, resolution_context)
        } else {
          Ok(redirected_uri)
        }
    }
}

impl fmt::Debug for BaseResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "BaseResolver", )
  }
}