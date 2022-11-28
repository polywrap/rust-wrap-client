use core::fmt;
use std::sync::Arc;

use crate::{
    error::Error,
    loader::Loader,
    resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    resolvers::uri_resolver::UriResolver,
    uri::Uri,
};
use async_trait::async_trait;

use super::{uri_resolver_like::UriResolverLike, uri_resolver_aggregator::UriResolverAggregator};

pub struct RecursiveResolver {
    resolver: Arc<dyn UriResolver>,
}

impl From<Vec<UriResolverLike>> for RecursiveResolver {
    fn from(resolver_likes: Vec<UriResolverLike>) -> Self {
        let resolvers = resolver_likes
            .into_iter()
            .map(|resolver_like| {
                let resolver: Arc<dyn UriResolver> = resolver_like.into();

                resolver
            })
            .collect();

        RecursiveResolver::new(
            Arc::new(
                UriResolverAggregator::new(
                    resolvers
                )
            )
        )
    }
}

impl RecursiveResolver {
    pub fn new(resolver: Arc<dyn UriResolver>) -> Self {
        Self { resolver }
    }

    async fn try_resolve_again_if_redirect(
        &self,
        result: Result<UriPackageOrWrapper, Error>,
        uri: &Uri,
        loader: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if let Ok(value) = &result {
            match value {
                UriPackageOrWrapper::Uri(result_uri) => {
                    if result_uri.clone().to_string() != uri.to_string() {
                        self.try_resolve_uri(result_uri, loader, resolution_context)
                            .await
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

impl From<UriResolverLike> for RecursiveResolver {
    fn from(resolver_like: UriResolverLike) -> Self {
        let resolver: Arc<dyn UriResolver> = resolver_like.into();
        RecursiveResolver::new(resolver)
    }
}

#[async_trait]
impl UriResolver for RecursiveResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        loader: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if resolution_context.is_resolving(uri) {
            //TODO: Handle this error type specifically
            Err(Error::ResolverError("Infinite loop error".to_string()))
        } else {
            resolution_context.start_resolving(uri);
            let resolver_result = self
                .resolver
                .try_resolve_uri(uri, loader, resolution_context)
                .await;

            let result = self
                .try_resolve_again_if_redirect(resolver_result, uri, loader, resolution_context)
                .await;

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