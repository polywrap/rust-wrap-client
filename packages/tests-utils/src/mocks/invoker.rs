use std::{
    collections::HashMap,
    sync::Arc,
};

use polywrap_core::{
    invoker::Invoker, macros::uri,
    uri::Uri,
};
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::InvokerContext;
use serde::{Deserialize, Serialize};
use wrap_manifest_schemas::versions::WrapManifest;

pub struct MockInvoker;

impl MockInvoker {
    // Manifest returned from invoke_raw when the tryResolveUri method is called
    pub fn manifest_from_try_resolve_uri_result() -> WrapManifest {
        wrap_manifest_schemas::versions::WrapManifest01 {
            abi: wrap_manifest_schemas::versions::WrapManifest01Abi {
                version: Some("1".to_string()),
                enum_types: None,
                env_type: None,
                imported_enum_types: None,
                imported_env_types: None,
                imported_module_types: None,
                imported_object_types: None,
                interface_types: None,
                module_type: None,
                object_types: None,
            },
            name: "mock".to_string(),
            version: "0.1".to_string(),
            type_: "wasm".to_string(),
        }
    }

    // URI returned from invoke_raw when the tryResolveUri method is called
    pub fn uri_from_try_resolve_uri() -> Uri {
        uri!("wrap://mock/resolved-uri")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
    pub uri: Option<String>,
    #[serde(with = "serde_bytes")]
    pub manifest: Option<Vec<u8>>,
}

impl Invoker for MockInvoker {
    fn invoke_raw(
        &self,
        _: &polywrap_core::uri::Uri,
        method: &str,
        _: Option<&[u8]>,
        _: Option<InvokerContext>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if method == "tryResolveUri" {
            let result: Vec<u8> = to_vec(&MaybeUriOrManifest {
                uri: Some(MockInvoker::uri_from_try_resolve_uri().to_string()),
                manifest: Some(to_vec(&MockInvoker::manifest_from_try_resolve_uri_result()).unwrap()),
            })
            .unwrap();

            Ok(result)
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
            uri!("mock/a"),
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
