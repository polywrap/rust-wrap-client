use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use polywrap_core::{
    invoker::Invoker, macros::uri, resolution::uri_resolution_context::UriResolutionContext,
    uri::Uri,
};

pub struct MockInvoker;

impl Invoker for MockInvoker {
    fn invoke_raw(
        &self,
        _: &polywrap_core::uri::Uri,
        method: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if method == "tryResolveUri" {
            Ok([
                132, 163, 97, 98, 105, 129, 167, 118, 101, 114, 115, 105, 111, 110, 161, 49, 164,
                110, 97, 109, 101, 164, 109, 111, 99, 107, 164, 116, 121, 112, 101, 164, 119, 97,
                115, 109, 167, 118, 101, 114, 115, 105, 111, 110, 163, 48, 46, 49,
            ]
            .to_vec())
        } else {
            Ok(vec![3])
        }
    }

    fn get_implementations(
        &self,
        _: &polywrap_core::uri::Uri,
    ) -> Result<Vec<polywrap_core::uri::Uri>, polywrap_core::error::Error> {
        Ok(vec![uri!("mock/a")])
    }

    fn get_interfaces(
        &self,
    ) -> Option<polywrap_core::interface_implementation::InterfaceImplementations> {
        Some(HashMap::from([(
            ("mock/a".to_string()),
            vec![uri!("mock/b")],
        )]))
    }

    fn get_env_by_uri(&self, _: &polywrap_core::uri::Uri) -> Option<Vec<u8>> {
        Some([0, 4].to_vec())
    }
}

pub fn get_mock_invoker() -> Arc<dyn Invoker> {
    Arc::new(MockInvoker {})
}
