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

        if let Some(manifest) = result.manifest {
            let package = WasmPackage::new(
                Arc::new(file_reader),
                Some(manifest),
                None
            );
            return Ok(UriPackageOrWrapper::Package(uri.clone(), Arc::new(package)));
        }

        if let Some(uri) = result.uri {
            return Ok(UriPackageOrWrapper::Uri(uri.try_into()?));
        }

        Ok(UriPackageOrWrapper::Uri(uri.clone()))
    }
}

impl fmt::Debug for UriResolverWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UriResolverWrapper: {:?}", self.implementation_uri)
    }
}