use std::sync::{Arc, Mutex};

use polywrap_core::{invoker::Invoker, resolution::uri_resolution_context::UriResolutionContext};

pub struct MockInvoker;

impl Invoker for MockInvoker {
    fn invoke_raw(
        &self,
        _: &polywrap_core::uri::Uri,
        _: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![3])
    }

    fn get_implementations(
        &self,
        _: &polywrap_core::uri::Uri,
    ) -> Result<Vec<polywrap_core::uri::Uri>, polywrap_core::error::Error> {
        Ok(vec![])
    }

    fn get_interfaces(
        &self,
    ) -> Option<polywrap_core::interface_implementation::InterfaceImplementations> {
        None
    }

    fn get_env_by_uri(&self, _: &polywrap_core::uri::Uri) -> Option<Vec<u8>> {
        None
    }
}

pub fn get_mock_invoker() -> Arc<dyn Invoker> {
    Arc::new(MockInvoker {})
}
