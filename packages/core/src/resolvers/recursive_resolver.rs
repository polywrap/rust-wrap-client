use async_trait::async_trait;
use polywrap_core::{uri_resolver::UriResolver, uri::Uri, loader::Loader, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}, error::Error};

use crate::{errors::Error, static_resolver::UriResolverLike};

pub struct RecursiveResolver {
  resolver: Box<dyn UriResolver>
}

impl RecursiveResolver {
  pub fn new(resolver: Box<dyn UriResolver>) -> Self {
    Self {
      resolver
    }
  }

  pub fn from(resolver: UriResolverLike) -> RecursiveResolver {
    todo!()
  }
}

#[async_trait]
impl UriResolver for RecursiveResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if resolution_context.is_resolving(uri) {
          //TODO: Handle this error type specifically
          return Err(Error::ResolverError(()))
        }
    }
}
