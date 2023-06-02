use polywrap_client::{
    core::{
        resolution::{
            uri_resolution_context::{UriPackageOrWrapper},
            uri_resolver::UriResolver,
        },
    },
    resolvers::static_resolver::StaticResolver,
};
use std::{collections::HashMap, sync::Arc, ops::DerefMut};

use crate::{uri::FFIUri, invoker::FFIInvoker};

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
      client: Arc<FFIInvoker>,
      resolution_context: Arc<FFIUriResolutionContext>,
  ) -> Box<dyn FFIUriPackageOrWrapper> {
      let mut uri_res_ctx_guard = resolution_context.0.lock().unwrap();

      let result = self
          .inner_resolver
          .try_resolve_uri(&uri.0, client, uri_res_ctx_guard.deref_mut())
          .unwrap();

      Box::new(result)
  }
}