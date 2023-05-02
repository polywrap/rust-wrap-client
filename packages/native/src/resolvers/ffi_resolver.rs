use polywrap_client::core::{resolvers::uri_resolver::UriResolver, uri::Uri};

use crate::client::FFILoader;
use std::{fmt::Debug, sync::Arc};

use super::uri_package_or_wrapper::FFIUriPackageOrWrapper;

pub trait FFIUriResolver: Send + Sync + Debug {
    fn wrap_try_resolve_uri(
      &self,
      uri: Arc<Uri>,
      client: Arc<FFILoader>
    ) -> Arc<FFIUriPackageOrWrapper>;
}

#[derive(Debug)]
pub struct FFIUriResolverWrapper(Arc<dyn FFIUriResolver>);

impl FFIUriResolverWrapper {
  pub fn new(uri_resolver: Arc<dyn FFIUriResolver>) -> Self {
    FFIUriResolverWrapper(uri_resolver)
  }
}

impl UriResolver for FFIUriResolverWrapper {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        client: std::sync::Arc<dyn polywrap_client::core::client::Loader>,
        _: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<polywrap_client::core::resolvers::uri_resolution_context::UriPackageOrWrapper, polywrap_client::core::error::Error> {
        let loader = FFIclient::new(loader);
        let result = self.0.wrap_try_resolve_uri(Arc::new(uri.clone()), Arc::new(loader));
        Ok(result.as_ref().clone().into())
    }
}

impl From<Box<dyn FFIUriResolver>> for FFIUriResolverWrapper {
    fn from(value: Box<dyn FFIUriResolver>) -> Self {
        FFIUriResolverWrapper(Arc::from(value))
    }
}