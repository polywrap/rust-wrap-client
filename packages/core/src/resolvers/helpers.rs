use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::{
    file_reader::FileReader,
    invoke::{Invoker, InvokeArgs},
    uri::Uri,
    error::Error,
    loader::Loader,
    interface_implementation::InterfaceImplementations
};
use polywrap_msgpack::{msgpack};

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
            "getFile",
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
    loader: Box<dyn Loader>,
) -> Result<Vec<Uri>, Error> {
    let mut implementation_uris: Vec<Uri> = vec![];

    if let Some(interfaces) = interfaces {
        let implementations_value = interfaces.get(&wrapper_uri.uri);
        if let Some(implementations) = implementations_value {
            for implementation in implementations.into_iter() {
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