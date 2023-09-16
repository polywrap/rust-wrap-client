use core::fmt;
use polywrap_core::{error::Error, invoker::Invoker, uri::Uri};
use std::sync::Arc;

use polywrap_core::resolution::{
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri_resolver::UriResolver,
};

use crate::uri_resolver_aggregator_base::UriResolverAggregatorBase;

pub struct UriResolverAggregator {
    name: Option<String>,
    resolvers: Vec<Arc<dyn UriResolver>>,
}

impl UriResolverAggregator {
    pub fn new(resolvers: Vec<Arc<dyn UriResolver>>) -> Self {
        let resolvers = resolvers.into_iter().map(Arc::from).collect();
        Self {
            name: None,
            resolvers,
        }
    }

    pub fn resolver_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

impl UriResolver for UriResolverAggregator {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let resolver_result =
            self.get_uri_resolvers(uri, invoker.as_ref(), resolution_context);

        if let Ok(resolvers) = resolver_result {
            self.try_resolve_uri_with_resolvers(uri, invoker, resolvers, resolution_context)
        } else {
            //TODO: verify this case.
            Err(Error::ResolutionError(
                "Failed to get URI resolvers".to_string(),
            ))
        }
    }
}

impl UriResolverAggregatorBase for UriResolverAggregator {
    fn get_resolver_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn get_step_description(&self, _: &Uri, _: &Result<UriPackageOrWrapper, Error>) -> String {
        if let Some(name) = self.get_resolver_name() {
            name
        } else {
            "UriResolverAggregator".to_string()
        }
    }

    fn get_uri_resolvers(
        &self,
        _: &Uri,
        _: &dyn Invoker,
        _: &mut UriResolutionContext,
    ) -> Result<Vec<Arc<dyn UriResolver>>, Error> {
        Ok(self.resolvers.clone())
    }
}

impl fmt::Debug for UriResolverAggregator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UriResolverAggregator\nResolvers: {:?}", self.resolvers)
    }
}

impl From<Vec<Box<dyn UriResolver>>> for UriResolverAggregator {
    fn from(resolvers: Vec<Box<dyn UriResolver>>) -> Self {
        UriResolverAggregator::new(resolvers.into_iter().map(Arc::from).collect())
    }
}

impl From<Vec<Arc<dyn UriResolver>>> for UriResolverAggregator {
    fn from(resolvers: Vec<Arc<dyn UriResolver>>) -> Self {
        UriResolverAggregator::new(resolvers)
    }
}
