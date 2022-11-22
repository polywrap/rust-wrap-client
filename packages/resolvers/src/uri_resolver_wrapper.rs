use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
  uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
  loader::Loader,
  uri::Uri,
  error::Error, wrapper::Wrapper, 
};

use crate::resolver_with_history::ResolverWithHistory;
use tokio::sync::Mutex;

pub struct UriResolverWrapper;

enum UriOrManifest {
  Uri(String),
  Manifest(Vec<u8>)
}

impl UriResolverWrapper {
  fn try_resolve_uri_with_implementation(
    &self,
    uri: Uri,
    implementation_uri: Uri,
    client: &dyn Loader,
    resolution_context: &mut UriResolutionContext
  ) -> Result<UriOrManifest, Error> {
    todo!()
  }

  async fn load_extension(
    &self,
    current_uri: Uri,
    resolver_extenion_uri: Uri,
    client: &dyn Loader,
    resolution_context: &mut UriResolutionContext
  ) -> Result<Arc<Mutex<dyn Wrapper>>, Error> {

    let result = client.try_resolve_uri(
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

    // Ok(Box::new())
  }
}

#[async_trait]
impl ResolverWithHistory for UriResolverWrapper {
    async fn _try_resolve_uri(
      &self, 
      uri: &Uri, 
      client: &dyn Loader, 
      resolution_context: &mut UriResolutionContext
    ) ->  Result<UriPackageOrWrapper, Error> {
        todo!() 
    }

    fn get_step_description(&self, uri: &Uri) -> String {
      format!("UriResolverWrapper - Implementation called: {}", uri.clone().uri)
    }
}