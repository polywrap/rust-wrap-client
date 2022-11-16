use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    loader::Loader,
    uri::Uri,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
    uri_resolver::UriResolver
};

#[async_trait]
pub trait ResolverWithHistory: Send + Sync {
  fn get_step_description(&self, uri: &Uri) -> String;
  async fn _try_resolve_uri(&self, uri: &Uri, loader: &dyn Loader, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error>;
}

#[async_trait]
impl UriResolver for dyn ResolverWithHistory {
  async fn try_resolve_uri(&self, uri: &Uri, loader: &dyn Loader, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error> {
    let result = self._try_resolve_uri(uri, loader, resolution_ctx).await;

    let resolution_step = UriResolutionStep {
      source_uri: uri.clone(),
      description: Some(self.get_step_description(uri)),
      sub_history: None,
      result: match &result {
          Ok(r) => Ok(r.clone()),
          Err(e) => Err(Error::ResolutionError(e.to_string()))
      }
    };

    resolution_ctx.track_step(resolution_step);

    result
  }
}
