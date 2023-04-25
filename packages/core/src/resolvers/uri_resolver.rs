use std::fmt::Debug;
use std::sync::Arc;

use crate::error::Error;
use crate::loader::Loader;
use crate::uri::Uri;
use super::package_resolver::PackageResolver;
use super::redirect_resolver::RedirectResolver;
use super::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext};
use super::uri_resolver_aggregator::UriResolverAggregator;
use super::uri_resolver_like::UriResolverLike;
use super::wrapper_resolver::WrapperResolver;

pub trait UriResolverHandler {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error>;
}

pub trait UriResolver: Send + Sync + Debug {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        loader: Arc<dyn Loader>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error>;
}

impl From<UriResolverLike> for Arc<dyn UriResolver> {
    fn from(resolver_like: UriResolverLike) -> Self {
        match resolver_like {
          UriResolverLike::ResolverLike(arr) => {
            let resolvers = arr.into_iter().map(|r| {
              let resolver: Arc<dyn UriResolver> = r.into();
              resolver
            }).collect();

            Arc::new(UriResolverAggregator::new(
              resolvers
            ))
          },
          UriResolverLike::Resolver(resolver) => Arc::from(resolver),
          UriResolverLike::Redirect(redirect) => {
            Arc::new(RedirectResolver {
              from: redirect.from,
              to: redirect.to
            })
          },
          UriResolverLike::Package(uri, package) => {
            Arc::new(PackageResolver {
              uri,
              package,
            })
          },
          UriResolverLike::Wrapper(uri, wrapper) => {
            Arc::new(WrapperResolver {
              uri,
              wrapper,
            })
          },
        }
    }
}