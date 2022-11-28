use core::fmt;

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    loader::Loader,
    uri::Uri,
    resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    resolvers::uri_resolver::UriResolver,
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

#[async_trait]
impl UriResolver for BaseResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        loader: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let redirected_uri = self.static_resolver.try_resolve_uri(uri, loader, resolution_context).await?;

        if let UriPackageOrWrapper::Uri(redirected_uri) = redirected_uri {
          self.fs_resolver.try_resolve_uri(&redirected_uri, loader, resolution_context).await
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