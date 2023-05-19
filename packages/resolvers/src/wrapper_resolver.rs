use core::fmt;
use std::sync::Arc;

use polywrap_core::resolution::uri_resolution_context::UriResolutionStep;
use polywrap_core::resolution::uri_resolver::UriResolver;
use polywrap_core::{uri::Uri, error::Error, wrapper::Wrapper, invoker::Invoker};

use polywrap_core::resolution::{
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
};

pub struct WrapperResolver {
    pub uri: Uri,
    pub wrapper: Arc<dyn Wrapper>,
}

impl WrapperResolver {}

impl UriResolver for WrapperResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: Arc<dyn Invoker>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let result: Result<UriPackageOrWrapper, Error> = { 
            if uri.to_string() != self.uri.to_string() {
                Ok(UriPackageOrWrapper::Uri(uri.clone()))
            } else {
                Ok(UriPackageOrWrapper::Wrapper(
                    uri.clone(),
                    self.wrapper.clone(),
                ))
            }
        };

        resolution_context.track_step(UriResolutionStep {
            source_uri: uri.clone(),
            description: Some(format!("Wrapper ({uri})")),
            result: match &result {
                Ok(r) => Ok(r.clone()),
                Err(e) => Err(Error::ResolutionError(e.to_string()))
            },
            sub_history: None,
        });

        result
    }
}

impl fmt::Debug for WrapperResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WrapperResolver: {}", self.uri)
    }
}
