use core::fmt;
use std::sync::{Arc};

use polywrap_core::{
  resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
  uri::Uri,
  error::Error, package::WrapPackage, invoker::Invoker, 
};
use polywrap_msgpack::{msgpack, decode};
use polywrap_wasm::wasm_package::{WasmPackage};
use serde::{Serialize,Deserialize};

use polywrap_core::{resolvers::resolver_with_history::ResolverWithHistory, resolvers::helpers::UriResolverExtensionFileReader};

pub struct UriResolverWrapper {
  pub implementation_uri: Uri
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
  pub uri: Option<String>,
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
    resolution_context: &mut UriResolutionContext
  ) -> Result<MaybeUriOrManifest, Error> {
      let result = invoker.invoke_raw(
          implementation_uri,
          "tryResolveUri", 
          Some(&msgpack!({
            "authority": uri.authority.as_str(),
            "path": uri.path.as_str(),
          })), 
          None, 
          Some(resolution_context)
      )?;

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

impl ResolverWithHistory for UriResolverWrapper {
    fn _try_resolve_uri(
      &self, 
      uri: &Uri, 
      invoker: Arc<dyn Invoker>, 
      resolution_context: &mut UriResolutionContext
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
          let wrapper = package.create_wrapper()?;
          return Ok(UriPackageOrWrapper::Wrapper(uri.clone(), wrapper));
      }

      let package = WasmPackage::new(
        Arc::new(file_reader), None, None
      );

      if package.get_manifest(None).is_ok() {
        return Ok(
          UriPackageOrWrapper::Package(uri.clone(), 
          Arc::new(package))
        );
      }

      if let Some(uri) = result.uri {
          return Ok(UriPackageOrWrapper::Uri(uri.try_into()?));
      }

      Ok(UriPackageOrWrapper::Uri(uri.clone()))
    }

    fn get_step_description(&self, uri: &Uri) -> String {
      format!("UriResolverWrapper - Implementation called: {}", uri.clone().uri)
    }
}

impl fmt::Debug for UriResolverWrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "UriResolverWrapper: {:?}", self.implementation_uri)
  }
}