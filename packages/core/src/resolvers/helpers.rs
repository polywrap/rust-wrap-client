use std::sync::Arc;
use async_trait::async_trait;
use crate::{
    file_reader::FileReader,
    invoke::{Invoker},
    uri::Uri,
    error::Error,
    loader::Loader,
    interface_implementation::InterfaceImplementations
};
use polywrap_msgpack::{msgpack};

fn combine_paths(a: &str, b: &str) -> String {
  let mut a = a.to_string();
  let mut b = b.to_string();

  a = a.replace('\\', "/");
  b = b.replace('\\', "/");

  if !a.ends_with('/') {
      a.push('/');
  };

  while b.chars().rev().last().unwrap() == '/' || b.chars().rev().last().unwrap() == '.' {
      b = b.split_off(1);
  }

  a.push_str(&b);

  a
}

pub struct UriResolverExtensionFileReader {
    pub resolver_extension_uri: Uri,
    pub wrapper_uri: Uri,
    pub invoker: Arc<dyn Invoker>
}

impl UriResolverExtensionFileReader {
    pub fn new(
        resolver_extension_uri: Uri, 
        wrapper_uri: Uri, 
        invoker: Arc<dyn Invoker>
    ) -> Self {
        UriResolverExtensionFileReader {
            resolver_extension_uri,
            wrapper_uri,
            invoker,
        } 
    } 
}

#[async_trait]
impl FileReader for UriResolverExtensionFileReader {
    async fn read_file(&self, file_path: &str) -> Result<Vec<u8>, Error> {
        let path = combine_paths(&self.wrapper_uri.path, file_path);

        let invoker_args = msgpack!({
            "path": path
        });
        // TODO: This vec<u8> isn't the file but the msgpack representation of it
        let result = self.invoker.invoke_raw(
            &self.resolver_extension_uri,
            "getFile",
            Some(&invoker_args),
            None,
            None
        ).await?;
        
        let result: Vec<u8> = polywrap_msgpack::decode(&result)?;
        Ok(result)
    }
}

pub async fn get_implementations(
    wrapper_uri: Uri,
    interfaces: Option<InterfaceImplementations>,
    _loader: Box<dyn Loader>,
) -> Result<Vec<Uri>, Error> {
    let mut implementation_uris: Vec<Uri> = vec![];

    if let Some(interfaces) = interfaces {
        let implementations_value = interfaces.get(&wrapper_uri.uri);
        if let Some(implementations) = implementations_value {
            for implementation in implementations.iter() {
                // TODO: Validate if implementation is already added
                // or if the implementation uri has redirect
                // by invoking loader.try_resolve_uri
                implementation_uris.push(implementation.clone());
            }
        }
    }

    Ok(implementation_uris)
    // for interface in interfaces.keys() {
    //     let mut fully_resolved_uri = implementation.clone();
    //     if let Some(l) = loader {
    //         let redirect_uri = l.try_resolve_uri(
    //             &implementation.clone(),
    //             resolution_context
    //         ).await;
    //     };

    //     if implementation_uris.contains(x)
    // }
}