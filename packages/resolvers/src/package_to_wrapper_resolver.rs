use std::fmt;
use std::sync::{Arc, Mutex};
use polywrap_core::{
    invoker::Invoker,
    uri::Uri,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
        uri_resolver::UriResolver
    },
    error::Error,
};
use crate::uri_resolver_aggregator::UriResolverAggregator;

/// A URI resolver that converts WrapPackages to a Wrappers as they pass through
pub struct PackageToWrapperResolver {
    resolver: Arc<dyn UriResolver>,
}

impl PackageToWrapperResolver {
    /// Creates a new `PackageToWrapperResolver`.
    ///
    /// # Returns
    ///
    /// * A new `PackageToWrapperResolver`.
    pub fn new(resolver: Arc<dyn UriResolver>) -> PackageToWrapperResolver {
        PackageToWrapperResolver { resolver }
    }

    fn package_to_wrapper(
        &self,
        uri_package_or_wrapper: UriPackageOrWrapper,
    ) -> Result<UriPackageOrWrapper, Error> {
        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri_value) => {
                Ok(UriPackageOrWrapper::Uri(uri_value))
            }

            UriPackageOrWrapper::Package(resolved_uri, wrap_package) => {
                match wrap_package.create_wrapper() {
                    Err(e) => Err(e),
                    Ok(wrapper) => {
                        Ok(UriPackageOrWrapper::Wrapper(resolved_uri, wrapper))
                    }
                }
            }

            UriPackageOrWrapper::Wrapper(resolved_uri, wrapper) => {
                Ok(UriPackageOrWrapper::Wrapper(resolved_uri, wrapper))
            }
        }
    }
}

impl UriResolver for PackageToWrapperResolver {
    /// Tries to resolve the given URI and returns the result,
    /// transforming WrapPackages to Wrappers if possible.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI to resolve.
    /// * `invoker` - The invoker of the resolution.
    /// * `resolution_context` - The context for the resolution.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the resolved `UriPackageOrWrapper` on success, or an exception on failure.
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {

        let result = self.resolver.try_resolve_uri(uri, invoker.clone(), resolution_context.clone());
        let final_result = match result {
            Ok(uri_package_or_wrapper) => self.package_to_wrapper(uri_package_or_wrapper),
            Err(_) => result,
        };

        resolution_context.lock().unwrap().track_step(
            UriResolutionStep {
                source_uri: uri.clone(),
                result: final_result.clone(),
                sub_history: None,
                description: Some("PackageToWrapperResolver".to_string()),
            }
        );

        return final_result;
    }
}

impl fmt::Debug for PackageToWrapperResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PackageToWrapperResolver")
    }
}

impl From<Vec<Box<dyn UriResolver>>> for PackageToWrapperResolver {
    fn from(resolvers: Vec<Box<dyn UriResolver>>) -> Self {
        PackageToWrapperResolver::from(
            UriResolverAggregator::from(resolvers)
        )
    }
}

impl From<UriResolverAggregator> for PackageToWrapperResolver {
    fn from(resolver: UriResolverAggregator) -> Self {
        PackageToWrapperResolver::new(
            Arc::new(resolver)
        )
    }
}

impl From<Box<dyn UriResolver>> for PackageToWrapperResolver {
    fn from(resolver: Box<dyn UriResolver>) -> Self {
        PackageToWrapperResolver::new(
            Arc::from(resolver)
        )
    }
}