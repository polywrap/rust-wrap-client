use core::fmt;
use std::{sync::Arc};

use crate::{error::Error, client::Client};

use super::{uri_resolver_like::UriResolverLike, uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}};

pub struct ResolverWithLoopGuard {
  pub resolver: Arc<dyn UriResolver>,
}

impl From<UriResolverLike> for ResolverWithLoopGuard {
    fn from(resolver_like: UriResolverLike) -> Self {
        let resolver: Arc<dyn UriResolver> = resolver_like.into();

        Self { resolver }
    }
}

impl UriResolver for ResolverWithLoopGuard {
    fn try_resolve_uri(&self, uri: &crate::uri::Uri, client: Arc<dyn Client>, resolution_context: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, Error> {
        if resolution_context.is_resolving(uri) {
          //TODO handle this error
          Err(Error::ResolverError("Infinite Loop".to_string()))
        } else {
          resolution_context.start_resolving(uri);

          let result = self.resolver.try_resolve_uri(uri, client, resolution_context);

          resolution_context.stop_resolving(uri);

          result
        }
    }
}

impl fmt::Debug for ResolverWithLoopGuard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "ResolverWithLoopGuard")
  }
}