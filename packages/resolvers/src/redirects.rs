// use async_trait::async_trait;
// use polywrap_core::{
//     error::Error,
//     loader::Loader,
//     uri::Uri,
//     uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
//     uri_resolver::UriResolver, client::UriRedirect,
// };

// #[async_trait]
// pub trait ResolverWithHistory: Send + Sync {
//   fn get_step_description(&self, uri: &Uri, result: Result<UriPackageOrWrapper, Error>) -> String;
//   async fn _try_resolve_uri(&mut self, uri: &Uri, loader: &dyn Loader, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error>;
// }

// #[async_trait]
// impl UriResolver for dyn ResolverWithHistory {
//   async fn try_resolve_uri(&mut self, uri: &Uri, loader: &dyn Loader, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error> {
//     let result = self._try_resolve_uri(uri, loader, resolution_ctx).await;

//     resolution_ctx.track_step(UriResolutionStep {
//       source_uri: uri.clone(),
//       description: Some(self.get_step_description(uri, result)),
//       sub_history: None,
//       result: if let Ok(result) = &result {
//         result.
//       }
//     });

//     result
//   }
// }

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
        _: &mut UriResolutionContext,
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
