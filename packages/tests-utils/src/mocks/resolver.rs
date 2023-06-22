use std::sync::{Arc, Mutex};

use polywrap_core::{
    error::Error,
    invoker::Invoker,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::UriResolver,
    },
    uri::Uri,
};

use super::{MockWrapper, DifferentMockWrapper};

pub fn get_mock_uri_package_or_wrapper() -> UriPackageOrWrapper {
    UriPackageOrWrapper::Wrapper("wrap/mock".try_into().unwrap(), Arc::new(MockWrapper {}))
}
#[derive(Debug)]
pub struct MockResolver;

impl UriResolver for MockResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: Arc<dyn Invoker>,
        _: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        if uri.to_string() == *"wrap://wrap/mock" {
            Ok(get_mock_uri_package_or_wrapper())
        } else {
            Err(Error::ResolutionError("Not Found".to_string()))
        }
    }
}

pub fn get_mock_resolver() -> Arc<MockResolver> {
    Arc::new(MockResolver {})
}

pub fn get_different_mock_uri_package_or_wrapper() -> UriPackageOrWrapper {
    UriPackageOrWrapper::Wrapper(
        "wrap/different-mock".try_into().unwrap(),
        Arc::new(DifferentMockWrapper {}),
    )
}

#[derive(Debug)]
pub struct DifferentMockResolver;

impl UriResolver for DifferentMockResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: Arc<dyn Invoker>,
        _: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        if uri.to_string() == *"wrap://wrap/mock" {
            Ok(get_mock_uri_package_or_wrapper())
        } else {
            Err(Error::ResolutionError("Not Found".to_string()))
        }
    }
}

pub fn get_different_mock_resolver() -> Arc<DifferentMockResolver> {
    Arc::new(DifferentMockResolver {})
}
