use polywrap_client::core::resolvers::uri_resolver::UriResolver;

use crate::loader::FFILoader;
use std::fmt::Debug;

use super::uri_package_or_wrapper::FFIUriPackageOrWrapper;

pub trait FFIUriResolver: Send + Sync + Debug {
    fn ffi_try_resolve_uri(
      &self,
      uri: &str,
      loader: FFILoader
    ) -> FFIUriPackageOrWrapper;
}

impl UriResolver for dyn FFIUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        loader: std::sync::Arc<dyn polywrap_client::core::loader::Loader>,
        _: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<polywrap_client::core::resolvers::uri_resolution_context::UriPackageOrWrapper, polywrap_client::core::error::Error> {
        let loader = FFILoader::new(loader);
        Ok(self.ffi_try_resolve_uri(&uri.to_string(), loader).into())
    }
}