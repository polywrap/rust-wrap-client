use polywrap_client::{
    core::{
        resolution::{
            uri_resolution_context::{UriPackageOrWrapper},
            uri_resolver::UriResolver,
        },
    },
    resolvers::static_resolver::StaticResolver,
};
use std::{collections::HashMap, sync::Arc};

use crate::{uri::FFIUri, invoker::{FFIInvoker, FFIInvokerWrapping}};

use super::{uri_package_or_wrapper::FFIUriPackageOrWrapper, resolution_context::FFIUriResolutionContext, ffi_resolver::FFIUriResolver};

#[derive(Debug)]
pub struct FFIStaticUriResolver {
    inner_resolver: StaticResolver,
}

impl FFIStaticUriResolver {
    pub fn new(uri_map: HashMap<String, Box<dyn FFIUriPackageOrWrapper>>) -> FFIStaticUriResolver {
        let uri_map: HashMap<String, UriPackageOrWrapper> = uri_map
            .into_iter()
            .map(|(uri, variant)| {
                let uri_package_or_wrapper: UriPackageOrWrapper = variant.into();
                (uri, uri_package_or_wrapper)
            })
            .collect();

        FFIStaticUriResolver {
            inner_resolver: StaticResolver::new(uri_map),
        }
    }
}

impl FFIUriResolver for FFIStaticUriResolver {
  fn try_resolve_uri(
      &self,
      uri: Arc<FFIUri>,
      invoker: Box<dyn FFIInvoker>,
      resolution_context: Arc<FFIUriResolutionContext>,
  ) -> Box<dyn FFIUriPackageOrWrapper> {
      let result = self
          .inner_resolver
          .try_resolve_uri(&uri.0, Arc::new(FFIInvokerWrapping(invoker)), resolution_context.0.clone())
          .unwrap();

      Box::new(result)
  }
}