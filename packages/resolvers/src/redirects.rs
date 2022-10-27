use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    loader::Loader,
    uri::Uri,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri_resolver::UriResolver, client::UriRedirect,
};

pub struct RedirectsResolver {
  redirects: Vec<UriRedirect>
}

impl RedirectsResolver {
  pub fn new(redirects: Vec<UriRedirect>) -> Self {
    Self {
      redirects
    }
  }
}

#[async_trait]
impl UriResolver for RedirectsResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: &dyn Loader,
        _: &UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let redirect = self.redirects.iter().find(|redirect| redirect.from == *uri);

        match redirect {
            Some(redirect) => {
              Ok(polywrap_core::uri_resolution_context::UriPackageOrWrapper::Uri(redirect.to.clone()))
            },
            None => {
                Ok(polywrap_core::uri_resolution_context::UriPackageOrWrapper::Uri(uri.clone()))
            }
        }
    }
}
