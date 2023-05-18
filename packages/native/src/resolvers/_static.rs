use std::{collections::HashMap, sync::Arc};

use polywrap_client::core::{resolvers::{static_resolver::StaticResolver, uri_resolution_context::UriPackageOrWrapper, uri_resolver::UriResolver}, invoker::Invoker};

use super::uri_package_or_wrapper::FFIUriPackageOrWrapper;

#[derive(Debug)]
pub struct FFIStaticUriResolver {
  inner_resolver: StaticResolver
}

impl FFIStaticUriResolver {
  pub fn new(uri_map: HashMap<String, Box<dyn FFIUriPackageOrWrapper>>) -> FFIStaticUriResolver {
    let uri_map: HashMap<String, UriPackageOrWrapper> = uri_map
        .into_iter()
        .map(|(uri, variant)| {
          let uri_package_or_wrapper: UriPackageOrWrapper = variant.as_ref().clone().into();
          (uri, uri_package_or_wrapper)
        })
        .collect();

    FFIStaticUriResolver {
      inner_resolver: StaticResolver::new(uri_map)
    }
  }
}

impl UriResolver for FFIStaticUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        self.inner_resolver.try_resolve_uri(uri, invoker, resolution_context)
    }
}