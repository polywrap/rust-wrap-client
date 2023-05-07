use core::fmt;
use std::sync::Arc;
use crate::{uri::Uri, invoker::Invoker};

use super::{
    resolver_with_history::ResolverWithHistory,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
};

pub struct RedirectResolver {
    pub from: Uri,
    pub to: Uri,
}

impl RedirectResolver {}

impl ResolverWithHistory for RedirectResolver {
    fn get_step_description(&self, _: &crate::uri::Uri) -> String {
        format!(
            "Redirect ({} - {})",
            self.from,
            self.to
        )
    }

    fn _try_resolve_uri(
        &self,
        uri: &Uri,
        _: Arc<dyn Invoker>,
        _: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, crate::error::Error> {
        if uri.to_string() != self.from.to_string() {
            Ok(UriPackageOrWrapper::Uri(uri.clone()))
        } else {
            Ok(UriPackageOrWrapper::Uri(self.to.clone()))
        }
    }
}

impl fmt::Debug for RedirectResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "RedirectResolver: {} - {}", self.from, self.to)
  }
}