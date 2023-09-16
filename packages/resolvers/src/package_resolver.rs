use core::fmt;
use std::sync::Arc;

use polywrap_core::{
    error::Error,
    invoker::Invoker,
    package::WrapPackage,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
        uri_resolver::UriResolver,
    },
    uri::Uri,
};

pub struct PackageResolver {
    pub uri: Uri,
    pub package: Arc<dyn WrapPackage>,
}

impl PackageResolver {}

impl UriResolver for PackageResolver {
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
                Ok(UriPackageOrWrapper::Package(
                    uri.clone(),
                    self.package.clone(),
                ))
            }
        };

        resolution_context
            .track_step(UriResolutionStep {
                source_uri: uri.clone(),
                description: Some(format!("Package ({})", self.uri)),
                result: match &result {
                    Ok(r) => Ok(r.clone()),
                    Err(e) => Err(Error::ResolutionError(e.to_string())),
                },
                sub_history: None,
            });

        result
    }
}

impl fmt::Debug for PackageResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PackageResolver: {}", self.uri)
    }
}
