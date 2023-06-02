use core::fmt;
use std::sync::{Arc, Mutex};
use polywrap_core::resolution::uri_resolution_context::UriResolutionStep;
use polywrap_core::resolution::uri_resolver::UriResolver;
use polywrap_core::{error::Error, uri::Uri, invoker::Invoker};

use polywrap_core::resolution::{
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
};

pub struct RedirectResolver {
    pub from: Uri,
    pub to: Uri,
}

impl RedirectResolver {}

impl UriResolver for RedirectResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let result: Result<UriPackageOrWrapper, Error> = { 
            if uri.to_string() != self.from.to_string() {
                Ok(UriPackageOrWrapper::Uri(uri.clone()))
            } else {
                Ok(UriPackageOrWrapper::Uri(self.to.clone()))
            }
        };

        resolution_context.lock().unwrap().track_step(UriResolutionStep {
            source_uri: uri.clone(),
            description: Some(  format!(
                "Redirect ({} - {})",
                self.from,
                self.to
            )),
            result: match &result {
                Ok(r) => Ok(r.clone()),
                Err(e) => Err(Error::ResolutionError(e.to_string()))
            },
            sub_history: None,
        });

        result
    }
}

impl fmt::Debug for RedirectResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "RedirectResolver: {} - {}", self.from, self.to)
  }
}
