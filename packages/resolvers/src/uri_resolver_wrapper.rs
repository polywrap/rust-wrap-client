use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
  uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
  invoke::{InvokeArgs},
  uri::Uri,
  error::Error, 
  wrapper::Wrapper, loader::Loader, package::WrapPackage, 
};
use polywrap_msgpack::{msgpack, decode};
use polywrap_wasm::wasm_package::{WasmPackage};
use serde::{Serialize,Deserialize};

use crate::{resolver_with_history::ResolverWithHistory, helpers::UriResolverExtensionFileReader};
use tokio::sync::Mutex;

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

  async fn try_resolve_uri_with_implementation(
    &self,
    uri: Uri,
    implementation_uri: Uri,
    loader: &dyn Loader,
    resolution_context: &mut UriResolutionContext
  ) -> Result<MaybeUriOrManifest, Error> {
      let mut sub_context = resolution_context.create_sub_context();
      let wrapper = self.load_extension(
        uri.clone(), 
        implementation_uri.clone(), 
        loader, 
        &mut sub_context
      ).await?;


      let mut env = None;
      if let Some(e) = loader.get_env_by_uri(&uri.clone()) {
          let e = e.to_owned();
          env = Some(e);
      };

      let invoke_args = InvokeArgs::Msgpack(msgpack!({
        "authority": uri.clone().authority,
        "path": uri.clone().path,
      }));

      let invoker = loader.get_invoker()?;
      let result = invoker.lock().await.invoke_wrapper(
          wrapper, 
          &uri.clone(), 
          "tryResolveUri", 
          Some(&invoke_args), 
          env, 
          Some(resolution_context)
      ).await?;
      decode::<MaybeUriOrManifest>(result.as_slice())
        .map_err(|e| Error::MsgpackError(format!("Failed to decode result: {}", e)))
  }

  async fn load_extension(
    &self,
    current_uri: Uri,
    resolver_extenion_uri: Uri,
    loader: &dyn Loader,
    resolution_context: &mut UriResolutionContext
  ) -> Result<Arc<Mutex<dyn Wrapper>>, Error> {

    let result = loader.try_resolve_uri(
      &current_uri,
      Some(resolution_context)
    ).await;

    if result.is_err() {
      let error = format!("Failed to resolver wrapper: {}", current_uri.clone().uri);
      return Err(Error::ResolutionError(error));
    }

    match result.unwrap() {
        UriPackageOrWrapper::Uri(uri) => {
          let error = format!(
            "While resolving {} with URI resolver extension {}, the extension could not be fully resolved. Last tried URI is {}", 
            current_uri.clone().uri, 
            resolver_extenion_uri.clone().uri,
            uri.clone().uri
          );
          return Err(Error::LoadWrapperError(error));
        },
        UriPackageOrWrapper::Package(_, package) => {
          let wrapper = package.lock().await
            .create_wrapper()
            .await
            .map_err(|e| Error::WrapperCreateError(e.to_string()))?;

            return Ok(wrapper);
          },
        UriPackageOrWrapper::Wrapper(_, wrapper) => {
          return Ok(wrapper);
        },
    }
  }
}

#[async_trait]
impl ResolverWithHistory for UriResolverWrapper {
    async fn _try_resolve_uri(
      &self, 
      uri: &Uri, 
      loader: &dyn Loader, 
      resolution_context: &mut UriResolutionContext
    ) ->  Result<UriPackageOrWrapper, Error> {
      let result = self.try_resolve_uri_with_implementation(
        uri.clone(), 
        self.implementation_uri.clone(), 
        loader, 
        resolution_context
      ).await?;

      
      let invoker = loader.get_invoker()?;
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

          let wrapper = package.create_wrapper().await?;

        return Ok(UriPackageOrWrapper::Wrapper(self.implementation_uri.clone(), wrapper));
      }

      let package = WasmPackage::new(
        Arc::new(file_reader), None, None
      );

      if package.get_manifest(None).await.is_ok() {
        return Ok(
          UriPackageOrWrapper::Package(self.implementation_uri.clone(), 
          Arc::new(Mutex::new(package)))
        );
      }

      if let Some(uri) = result.uri {
          return Ok(UriPackageOrWrapper::Uri(Uri::from_string(uri.clone().as_str())?));
      }

      Ok(UriPackageOrWrapper::Uri(uri.clone()))
    }

    fn get_step_description(&self, uri: &Uri) -> String {
      format!("UriResolverWrapper - Implementation called: {}", uri.clone().uri)
    }
}