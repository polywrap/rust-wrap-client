use polywrap_client::{core::resolvers::{uri_resolution_context::UriPackageOrWrapper, uri_resolver::UriResolver}, resolvers::extendable_uri_resolver::ExtendableUriResolver};

#[derive(Debug)]
pub struct FFIExtendableUriResolver {
  inner_resolver: ExtendableUriResolver
}

impl FFIExtendableUriResolver {
  pub fn new(name: Option<String>) -> FFIExtendableUriResolver {
    FFIExtendableUriResolver {
      inner_resolver: ExtendableUriResolver::new(name)
    }
  }
}

impl UriResolver for FFIExtendableUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        loader: std::sync::Arc<dyn polywrap_client::core::loader::Loader>,
        resolution_context: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        self.inner_resolver.try_resolve_uri(uri, loader, resolution_context)
    }
}