use std::sync::Arc;

use crate::{
    client::CoreClient, error::Error, file_reader::FileReader,
    interface_implementation::InterfaceImplementations, invoker::Invoker, uri::Uri,
};
use polywrap_msgpack_serde::to_vec;
use serde::Serialize;
use serde_bytes::ByteBuf;

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
    invoker: Arc<dyn Invoker>,
}

impl UriResolverExtensionFileReader {
    pub fn new(resolver_extension_uri: Uri, wrapper_uri: Uri, invoker: Arc<dyn Invoker>) -> Self {
        UriResolverExtensionFileReader {
            resolver_extension_uri,
            wrapper_uri,
            invoker,
        }
    }
}

#[derive(Serialize)]
struct GetFileArgs {
    path: String,
}

impl FileReader for UriResolverExtensionFileReader {
    fn read_file(&self, file_path: &str) -> Result<Vec<u8>, Error> {
        let path = combine_paths(&self.wrapper_uri.path(), file_path);
        let args = GetFileArgs { path };
        let invoker_args = to_vec(&args).unwrap();
        // TODO: This vec<u8> isn't the file but the msgpack representation of it
        let result = self.invoker.invoke_raw(
            &self.resolver_extension_uri,
            "getFile",
            Some(&invoker_args),
            None,
        )?;

        let result: ByteBuf = polywrap_msgpack_serde::from_slice(&result)?;
        Ok(result.into_vec())
    }
}

pub fn get_implementations(
    wrapper_uri: &Uri,
    interfaces: Option<InterfaceImplementations>,
    _: &dyn CoreClient,
) -> Result<Vec<Uri>, Error> {
    let mut implementation_uris: Vec<Uri> = vec![];

    if let Some(interfaces) = interfaces {
        let implementations_value = interfaces.get(&wrapper_uri);
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
}

pub fn get_env_from_resolution_path(
    resolution_path: &[Uri],
    client: &dyn CoreClient,
) -> Option<Vec<u8>> {
    for uri in resolution_path.iter() {
        let env = client.get_env_by_uri(uri);

        if env.is_some() {
            return env;
        }
    }

    None
}
