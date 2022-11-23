use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use polywrap_core::{
    file_reader::FileReader, 
    invoke::{Invoker, InvokeArgs}, uri::Uri,
    error::Error
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
            resolver_extension_uri: resolver_extension_uri.clone(),
            wrapper_uri: wrapper_uri.clone(),
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