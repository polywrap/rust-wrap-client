
use std::{fmt::Debug, sync::Arc};

use polywrap_client::core::{resolution::{uri_resolver::UriResolver, uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}}, invoker::Invoker};

use crate::{invoker::FFIInvoker, uri::FFIUri};

use super::uri_package_or_wrapper::{FFIUriPackageOrWrapper};

pub trait FFIUriResolver: Send + Sync + Debug {
    fn try_resolve_uri(
      &self,
      uri: Arc<FFIUri>,
      client: Arc<FFIInvoker>
    ) -> Box<dyn FFIUriPackageOrWrapper>;
}

#[derive(Debug)]
pub struct ExtUriResolver(pub Box<dyn FFIUriResolver>);

impl UriResolver for ExtUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        invoker: Arc<dyn Invoker>,
        _: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        let result = self.0.try_resolve_uri(Arc::new(uri.clone().into()), Arc::new(FFIInvoker::new(invoker)));
        Ok(result.into())
    }
}
