use core::fmt;
use std::sync::Arc;

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

// impl From<Vec<Arc<dyn UriResolver>>> for RecursiveResolver {
//     fn from(resolvers: Vec<Arc<dyn UriResolver>>) -> Self {
//         RecursiveResolver::new(
//             Arc::new(
//                 UriResolverAggregator::new(
//                     resolvers.into_iter().map(|resolver| resolver as Arc<dyn UriResolver>).collect()
//                 )
//             )
//         )
//     }
// }

impl From<Vec<Box<dyn UriResolver>>> for RecursiveResolver {
    fn from(resolvers: Vec<Box<dyn UriResolver>>) -> Self {
        let aggregator = UriResolverAggregator::new(resolvers);
        RecursiveResolver::new(
            Arc::new(aggregator)
        )
    }
}

impl From<Arc<dyn UriResolver>> for RecursiveResolver {
    fn from(resolver: Arc<dyn UriResolver>) -> Self {
        RecursiveResolver::new(resolver)
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
        resolution_context: &mut UriResolutionContext,
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
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if resolution_context.is_resolving(uri) {
            //TODO: Handle this error type specifically
            Err(Error::ResolverError("Infinite loop error".to_string()))
        } else {
            resolution_context.start_resolving(uri);
            let resolver_result = self
                .resolver
                .try_resolve_uri(uri, invoker.clone(), resolution_context);

            let result = self
                .try_resolve_again_if_redirect(resolver_result, uri, invoker, resolution_context);

            resolution_context.stop_resolving(uri);

            result
        }
    }
}

impl fmt::Debug for RecursiveResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "RecursiveResolver")
  }
}