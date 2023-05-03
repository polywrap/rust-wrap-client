use core::fmt;
use std::{sync::Arc};
use crate::{error::Error, loader::Loader, uri::Uri};

use super::{
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri_resolver::UriResolver,
    uri_resolver_aggregator_base::UriResolverAggregatorBase,
    uri_resolver_like::UriResolverLike,
};

pub struct UriResolverAggregator {
    name: Option<String>,
    resolvers: Vec<Arc<dyn UriResolver>>,
}

impl UriResolverAggregator {
    pub fn new(resolvers: Vec<Arc<dyn UriResolver>>) -> Self {
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

impl From<Vec<UriResolverLike>> for UriResolverAggregator {
    fn from(resolver_likes: Vec<UriResolverLike>) -> Self {
        let resolvers = resolver_likes
            .into_iter()
            .map(|resolver_like| {
                let resolver: Arc<dyn UriResolver> = resolver_like.into();

                resolver
            })
            .collect();

        UriResolverAggregator::new(resolvers)
    }
}

impl UriResolver for UriResolverAggregator {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        loader: Arc<dyn Loader>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let resolver_result = self
            .get_uri_resolvers(uri, loader.clone(), resolution_context);

        if let Ok(resolvers) = resolver_result {
          self.try_resolve_uri_with_resolvers(uri, loader, resolvers, resolution_context)
        } else {
          //TODO: verify this case.
          Err(Error::ResolutionError("Failed to get URI resolvers".to_string()))
        }
    }
}

impl UriResolverAggregatorBase for UriResolverAggregator {
    fn get_resolver_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn get_step_description(
        &self,
        _: &Uri,
        _: &Result<UriPackageOrWrapper, Error>,
    ) -> String {
        if let Some(name) = self.get_resolver_name() {
            name
        } else {
            "UriResolverAggregator".to_string()
        }
    }

    fn get_uri_resolvers(
        &self,
        _: &Uri,
        _: Arc<dyn Loader>,
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