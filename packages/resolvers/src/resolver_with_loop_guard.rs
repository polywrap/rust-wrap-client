use core::fmt;
use std::sync::{Arc, Mutex};

use polywrap_core::{
  error::Error, invoker::Invoker, uri::Uri, 
  resolution::{uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}}
};

pub struct ResolverWithLoopGuard {
    pub resolver: Arc<dyn UriResolver>,
}

impl UriResolver for ResolverWithLoopGuard {
    fn try_resolve_uri(&self, uri: &Uri, invoker: Arc<dyn Invoker>, resolution_context: Arc<Mutex<UriResolutionContext>>) -> Result<UriPackageOrWrapper, Error> {
        if resolution_context.lock().unwrap().is_resolving(uri) {
          //TODO handle this error
          Err(Error::ResolverError("Infinite Loop".to_string()))
        } else {
          resolution_context.lock().unwrap().start_resolving(uri);

          let result = self.resolver.try_resolve_uri(uri, invoker, resolution_context.clone());

          resolution_context.lock().unwrap().stop_resolving(uri);

          result
        }
    }
}

impl fmt::Debug for ResolverWithLoopGuard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "ResolverWithLoopGuard")
  }
}