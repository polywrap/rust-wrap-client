use std::sync::Arc;
use tokio::sync::Mutex;
use polywrap_core::{
    file_reader::FileReader, 
    invoke::Invoker, uri::Uri
};

pub struct UriResolverExtensionFileReader {
    pub resolver_extension_uri: Uri,
    pub wrapper_uri: Uri,
    pub invoker: Arc<Mutex<dyn Invoker>>
}

impl UriResolverExtensionFileReader {
    pub fn new(
        resolver_with_history: Uri, 
        wrapper_uri: Uri, 
        invoker: Arc<Mutex<dyn Invoker>>
    ) -> Self {
        UriResolverExtensionFileReader {
            resolver_extension_uri: resolver_with_history.clone(),
            wrapper_uri: wrapper_uri.clone(),
            invoker,
        } 
    } 
}

impl FileReader for UriResolverExtensionFileReader {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, polywrap_core::error::Error> {
        todo!()
    }
}