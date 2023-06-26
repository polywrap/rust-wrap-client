use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use polywrap_core::{
    invoker::Invoker, macros::uri, resolution::uri_resolution_context::UriResolutionContext,
    uri::Uri,
};
use polywrap_msgpack::encode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
    pub uri: Option<String>,
    #[serde(with = "serde_bytes")]
    pub manifest: Option<Vec<u8>>,
}

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
            let manifest = wrap_manifest_schemas::versions::WrapManifest01 {
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
            };
            let manifest = encode(&manifest).unwrap();

            let result: Vec<u8> = encode(&MaybeUriOrManifest {
                uri: Some("wrap://mock/resolved-uri".to_string()),
                manifest: Some(manifest),
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
