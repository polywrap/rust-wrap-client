use core::fmt;
use std::sync::{Arc, Mutex};

use polywrap_core::{
    resolution::{uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep}, uri_resolver::UriResolver, helpers::UriResolverExtensionFileReader},
    uri::Uri,
    error::Error, invoker::Invoker, 
};
use polywrap_msgpack::{msgpack, decode};
use polywrap_wasm::wasm_package::WasmPackage;
use serde::{Serialize,Deserialize};

pub struct UriResolverWrapper {
    pub implementation_uri: Uri
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
    pub uri: Option<String>,
    #[serde(with = "serde_bytes")]
    pub manifest: Option<Vec<u8>>,
}

impl UriResolverWrapper {
    pub fn new(implementation_uri: Uri) -> Self {
        UriResolverWrapper { implementation_uri }
    }

    fn try_resolve_uri_with_implementation(
        &self,
        uri: &Uri,
        implementation_uri: &Uri,
        invoker: &dyn Invoker,
        resolution_context: Arc<Mutex<UriResolutionContext>>
    ) -> Result<MaybeUriOrManifest, Error> {
        let resolver_extension_context = resolution_context.lock().unwrap().create_sub_context();
        let resolver_extension_context = Arc::new(Mutex::new(resolver_extension_context));
        let result = invoker.invoke_raw(
            implementation_uri,
            "tryResolveUri", 
            Some(&msgpack!({
                "authority": uri.authority.as_str(),
                "path": uri.path.as_str(),
            })), 
            None, 
            Some(resolver_extension_context.clone())
        );

        resolution_context.lock().unwrap().track_step(UriResolutionStep {
            source_uri: uri.clone(),
            result: match result.clone() {
                Ok(_) => Ok(UriPackageOrWrapper::Uri(implementation_uri.clone())),
                Err(e) => Err(e),
            },
            description: Some(format!("ResolverExtension({implementation_uri})")),
            sub_history: Some(resolver_extension_context.lock().unwrap().get_history().clone())
        });

        let result = result?;

        if result.is_empty() {
            Ok(MaybeUriOrManifest {
                uri: None,
                manifest: None
            })
        } else {
            Ok(decode::<MaybeUriOrManifest>(result.as_slice())?)
        }
  }
}

impl UriResolver for UriResolverWrapper {
    fn try_resolve_uri(
        &self, 
        uri: &Uri, 
        invoker: Arc<dyn Invoker>, 
        resolution_context: Arc<Mutex<UriResolutionContext>>
    ) ->  Result<UriPackageOrWrapper, Error> {
        let result = self.try_resolve_uri_with_implementation(
            uri, 
            &self.implementation_uri, 
            invoker.as_ref(), 
            resolution_context
        )?;

        let file_reader = UriResolverExtensionFileReader::new(
            self.implementation_uri.clone(),
            uri.clone(),
            invoker
        );

        let uri = if let Some(resolved_uri) = result.uri {
            Uri::try_from(resolved_uri)?
        } else {
            uri.clone()
        };

        if let Some(manifest) = result.manifest {
            let package = WasmPackage::new(
                Arc::new(file_reader),
                Some(manifest),
                None
            );
            return Ok(UriPackageOrWrapper::Package(
                uri.clone(), 
                Arc::new(package
            )));
        }

        Ok(UriPackageOrWrapper::Uri(uri))
    }
}

impl fmt::Debug for UriResolverWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UriResolverWrapper: {:?}", self.implementation_uri)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use polywrap_core::{uri::Uri, resolution::{uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}, uri_resolver::UriResolver}, invoker::Invoker, package::GetManifestOptions};
    use polywrap_msgpack::rmp_serde::encode;

    use super::{UriResolverWrapper, MaybeUriOrManifest};

    struct MockInvoker {}

    impl Invoker for MockInvoker {
        fn invoke_raw(
            &self,
            _uri: &Uri,
            _method_name: &str,
            _args: Option<&[u8]>,
            _env: Option<&[u8]>,
            _invocation_context: Option<Arc<Mutex<UriResolutionContext>>>,
        ) -> Result<Vec<u8>, polywrap_core::error::Error> {
            let manifest = wrap_manifest_schemas::versions::WrapManifest01 {
                abi: wrap_manifest_schemas::versions::WrapManifest01Abi {
                    ..Default::default()
                },
                name: "mock".to_string(),
                version: "0.1".to_string(),
                type_: "wasm".to_string()
            };
            let manifest = encode::to_vec_named(&manifest).unwrap();

            let result: Vec<u8> = encode::to_vec_named(&MaybeUriOrManifest {
                uri: Some("wrap://mock/resolved-uri".to_string()),
                manifest: Some(manifest)
            }).unwrap();

            Ok(result)
        }

        fn get_implementations(&self, _uri: &Uri) -> Result<Vec<Uri>, polywrap_core::error::Error> {
            Ok(vec![])
        }

        fn get_interfaces(&self) -> Option<polywrap_core::interface_implementation::InterfaceImplementations> {
            None
        }

        fn get_env_by_uri(&self, _uri: &Uri) -> Option<Vec<u8>> {
            None
        }
    }

    #[test]
    fn sanity() {
        let resolver_extension = UriResolverWrapper {
            implementation_uri: Uri::try_from("wrap://mock/extension-uri").unwrap()
        };

        let result = resolver_extension.try_resolve_uri(
            &Uri::try_from("wrap://mock/uri-to-resolver").unwrap(),
            Arc::new(MockInvoker {}),
            Arc::new(Mutex::new(UriResolutionContext::new()))
        ).unwrap();
        
        let expected_manifest = wrap_manifest_schemas::versions::WrapManifest01 {
            abi: wrap_manifest_schemas::versions::WrapManifest01Abi {
                ..Default::default()
            },
            name: "mock".to_string(),
            version: "0.1".to_string(),
            type_: "wasm".to_string()
        };

        match result {
            UriPackageOrWrapper::Package(uri, package) => {
                assert_eq!(uri, Uri::try_from("wrap://mock/resolved-uri").unwrap());
                assert_eq!(package.get_manifest(Some(&GetManifestOptions {
                    no_validate: true
                })).unwrap(), expected_manifest);
            },
            _ => panic!("Expected UriPackageOrWrapper::Package")
        }
    }
}