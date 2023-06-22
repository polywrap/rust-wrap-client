use core::fmt;
use std::sync::{Arc, Mutex};

use polywrap_core::{
    error::Error,
    invoker::Invoker,
    resolution::{
        helpers::UriResolverExtensionFileReader,
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
        uri_resolver::UriResolver,
    },
    uri::Uri,
};
use polywrap_msgpack::{decode, msgpack};
use polywrap_wasm::wasm_package::WasmPackage;
use serde::{Deserialize, Serialize};

pub struct UriResolverWrapper {
    pub implementation_uri: Uri,
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
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<MaybeUriOrManifest, Error> {
        let resolver_extension_context = resolution_context.lock().unwrap().create_sub_context();
        let resolver_extension_context = Arc::new(Mutex::new(resolver_extension_context));
        let result = invoker.invoke_raw(
            implementation_uri,
            "tryResolveUri",
            Some(&msgpack!({
                "authority": uri.authority(),
                "path": uri.path(),
            })),
            None,
            Some(resolver_extension_context.clone()),
        );

        resolution_context
            .lock()
            .unwrap()
            .track_step(UriResolutionStep {
                source_uri: uri.clone(),
                result: match result.clone() {
                    Ok(_) => Ok(UriPackageOrWrapper::Uri(implementation_uri.clone())),
                    Err(e) => Err(e),
                },
                description: Some(format!("ResolverExtension({implementation_uri})")),
                sub_history: Some(
                    resolver_extension_context
                        .lock()
                        .unwrap()
                        .get_history()
                        .clone(),
                ),
            });

        let result = result?;

        if result.is_empty() {
            Ok(MaybeUriOrManifest {
                uri: None,
                manifest: None,
            })
        } else {
            let result = decode::<Option<MaybeUriOrManifest>>(result.as_slice())?;

            let result = result.unwrap_or(MaybeUriOrManifest {
                uri: None,
                manifest: None,
            });

            Ok(result)
        }
    }
}

impl UriResolver for UriResolverWrapper {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let result = self.try_resolve_uri_with_implementation(
            uri,
            &self.implementation_uri,
            invoker.as_ref(),
            resolution_context,
        )?;

        let file_reader = UriResolverExtensionFileReader::new(
            self.implementation_uri.clone(),
            uri.clone(),
            invoker,
        );

        let uri = if let Some(resolved_uri) = result.uri {
            Uri::try_from(resolved_uri)?
        } else {
            uri.clone()
        };

        if let Some(manifest) = result.manifest {
            let package = WasmPackage::from_file_reader(Arc::new(file_reader), Some(manifest));
            return Ok(UriPackageOrWrapper::Package(uri.clone(), Arc::new(package)));
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

    use polywrap_core::{
        macros::uri,
        package::GetManifestOptions,
        resolution::{
            uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
            uri_resolver::UriResolver,
        },
        uri::Uri,
    };
    use polywrap_tests_utils::mocks::MockInvoker;

    use super::UriResolverWrapper;

    #[test]
    fn sanity() {
        let resolver_extension = UriResolverWrapper {
            implementation_uri: uri!("wrap://mock/extension-uri"),
        };

        let result = resolver_extension
            .try_resolve_uri(
                &uri!("wrap://mock/uri-to-resolver"),
                Arc::new(MockInvoker {}),
                Arc::new(Mutex::new(UriResolutionContext::new())),
            )
            .unwrap();

        let expected_manifest = wrap_manifest_schemas::versions::WrapManifest01 {
            abi: wrap_manifest_schemas::versions::WrapManifest01Abi {
                ..Default::default()
            },
            name: "mock".to_string(),
            version: "0.1".to_string(),
            type_: "wasm".to_string(),
        };

        match result {
            UriPackageOrWrapper::Package(uri, package) => {
                assert_eq!(uri, uri!("wrap://mock/resolved-uri"));
                assert_eq!(
                    package
                        .get_manifest(Some(&GetManifestOptions { no_validate: true }))
                        .unwrap(),
                    expected_manifest
                );
            }
            _ => panic!("Expected UriPackageOrWrapper::Package"),
        }
    }
}
