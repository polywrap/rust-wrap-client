use std::{
    collections::HashMap,
    sync::Arc,
};

use polywrap_core::{
    client::CoreClient,
    invoker::Invoker,
    macros::uri,
    resolution::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri::Uri,
    uri_resolver_handler::UriResolverHandler,
    wrap_invoker::WrapInvoker,
    wrap_loader::WrapLoader,
    wrapper::Wrapper,
};

use super::get_mock_wrapper;

pub struct MockClient;

impl Invoker for MockClient {
    fn invoke_raw(
        &self,
        _: &Uri,
        _: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![5])
    }

    fn get_implementations(&self, _: &Uri) -> Result<Vec<Uri>, polywrap_core::error::Error> {
        Ok(vec![uri!("mock/c")])
    }

    fn get_interfaces(
        &self,
    ) -> Option<polywrap_core::interface_implementation::InterfaceImplementations> {
        Some(HashMap::from([(
            uri!("mock/c"),
            vec![uri!("mock/d")],
        )]))
    }

    fn get_env_by_uri(&self, _: &Uri) -> Option<Vec<u8>> {
        Some([4, 8].to_vec())
    }
}

impl WrapLoader for MockClient {
    fn load_wrapper(
        &self,
        _: &Uri,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<dyn Wrapper>, polywrap_core::error::Error> {
        Ok(get_mock_wrapper())
    }
}

impl WrapInvoker for MockClient {
    fn invoke_wrapper_raw(
        &self,
        _: &dyn Wrapper,
        _: &Uri,
        _: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![6])
    }
}

impl UriResolverHandler for MockClient {
    fn try_resolve_uri(
        &self,
        _: &Uri,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, polywrap_core::error::Error> {
        Ok(UriPackageOrWrapper::Uri(uri!("mock/b")))
    }
}

impl CoreClient for MockClient {}

pub fn get_mock_client() -> Arc<dyn CoreClient> {
    Arc::new(MockClient {})
}
