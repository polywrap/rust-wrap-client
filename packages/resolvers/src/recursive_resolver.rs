use core::fmt;
use std::sync::{Arc, Mutex};

use polywrap_core::{
    error::Error,
    resolution::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    resolution::uri_resolver::UriResolver,
    uri::Uri, invoker::Invoker,
};

use crate::uri_resolver_aggregator::UriResolverAggregator;

pub struct RecursiveResolver {
    resolver: Arc<dyn UriResolver>,
}

impl From<Vec<Box<dyn UriResolver>>> for RecursiveResolver {
    fn from(resolvers: Vec<Box<dyn UriResolver>>) -> Self {
        RecursiveResolver::from(
            UriResolverAggregator::from(resolvers)
        )
    }
}

impl From<UriResolverAggregator> for RecursiveResolver {
    fn from(resolver: UriResolverAggregator) -> Self {
        RecursiveResolver::new(
            Arc::new(resolver)
        )
    }
}

impl From<Box<dyn UriResolver>> for RecursiveResolver {
    fn from(resolver: Box<dyn UriResolver>) -> Self {
        RecursiveResolver::new(Arc::from(resolver))
    }
}

impl RecursiveResolver {
    pub fn new(resolver: Arc<dyn UriResolver>) -> Self {
        Self { resolver }
    }

    fn try_resolve_again_if_redirect(
        &self,
        result: Result<UriPackageOrWrapper, Error>,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        if let Ok(value) = &result {
            match value {
                UriPackageOrWrapper::Uri(result_uri) => {
                    if result_uri.clone().to_string() != uri.to_string() {
                        self.try_resolve_uri(result_uri, invoker, resolution_context)
                    } else {
                        result
                    }
                }
                _ => result,
            }
        } else {
            result
        }
    }
}

impl UriResolver for RecursiveResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        if resolution_context.lock().unwrap().is_resolving(uri) {
            //TODO: Handle this error type specifically
            Err(Error::ResolverError("Infinite loop error".to_string()))
        } else {
            resolution_context.lock().unwrap().start_resolving(uri);
            let resolver_result = self
                .resolver
                .try_resolve_uri(uri, invoker.clone(), resolution_context.clone());

            let result = self
                .try_resolve_again_if_redirect(resolver_result, uri, invoker, resolution_context.clone());

            resolution_context.lock().unwrap().stop_resolving(uri);

            result
        }
    }
}

impl fmt::Debug for RecursiveResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "RecursiveResolver")
  }
}