use std::{fmt::Debug};

use crate::{
    error::Error,
    uri::Uri,
    resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
    resolvers::uri_resolver::UriResolver, client::Client
};

pub trait ResolverWithHistory: Send + Sync {
  fn get_step_description(&self, uri: &Uri) -> String;
  fn _try_resolve_uri(&self, uri: &Uri, client: &dyn Client, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error>;
}

impl<T: ResolverWithHistory + Debug> UriResolver for T {
  fn try_resolve_uri(&self, uri: &Uri, client: &dyn Client, resolution_ctx: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error> {
    let result = self._try_resolve_uri(uri, client, resolution_ctx);

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
