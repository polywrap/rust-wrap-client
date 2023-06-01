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

use super::MockWrapper;

#[derive(Debug)]
pub struct MockResolver {}

impl UriResolver for MockResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: Arc<dyn Invoker>,
        _: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        if uri.to_string() == *"wrap://ens/mock.eth" {
            Ok(UriPackageOrWrapper::Wrapper(
                "wrap://ens/mock.eth".try_into().unwrap(),
                Arc::new(MockWrapper {}),
            ))
        } else {
            Err(Error::ResolutionError("Not Found".to_string()))
        }
    }
}

pub fn get_mock_resolver() -> Arc<MockResolver> {
    Arc::new(MockResolver {})
}
