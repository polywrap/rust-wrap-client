use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::{
    file_reader::FileReader, 
    invoke::{Invoker, InvokeArgs}, uri::Uri,
    error::Error, loader::Loader, interface_implementation::InterfaceImplementations
};
use polywrap_msgpack::{msgpack};

use super::uri_resolution_context::UriResolutionContext;

pub struct UriResolverExtensionFileReader {
    pub resolver_extension_uri: Uri,
    pub wrapper_uri: Uri,
    pub invoker: Arc<Mutex<dyn Invoker>>
}

impl UriResolverExtensionFileReader {
    pub fn new(
        resolver_extension_uri: Uri, 
        wrapper_uri: Uri, 
        invoker: Arc<Mutex<dyn Invoker>>
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
    async fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
        let invoker_args = InvokeArgs::Msgpack(msgpack!({
            "path": path
        }));
        let result = self.invoker.lock().await.invoke(
            &self.resolver_extension_uri,
            "get_file",
            Some(&invoker_args),
            None,
            None
        ).await?; 
        Ok(result)
    }
}

pub async fn get_implementations(
    wrapper_uri: Uri,
    interfaces: Option<InterfaceImplementations>,
    loader: Option<&dyn Loader>,
    resolution_context: Option<&mut UriResolutionContext>
) -> Result<Vec<Uri>, Error> {
    let mut implementation_uris: Vec<Uri> = vec![];

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

    Ok(vec![])
}