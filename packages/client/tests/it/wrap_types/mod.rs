use std::{collections::HashMap, sync::Arc};

use polywrap_client::client::PolywrapClient;
use polywrap_core::{
    client::ClientConfig, file_reader::SimpleFileReader,
    resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri,
};
use polywrap_resolvers::{
    base_resolver::BaseResolver, simple_file_resolver::FilesystemResolver,
    static_resolver::StaticResolver,
};

pub mod asyncify;
pub mod bigint;
pub mod bignumber;
pub mod bytes;
pub mod r#enum;
pub mod invalid_invokes;
pub mod json;
pub mod map;
pub mod numbers;
pub mod object;

pub fn get_client(
    static_resolvers: Option<HashMap<Uri, UriPackageOrWrapper>>,
) -> PolywrapClient {
    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let static_resolver = if let Some(s) = static_resolvers {
        s
    } else {
        HashMap::new()
    };

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(static_resolver)),
    );

    let config = ClientConfig {
        resolver: Arc::new(base_resolver),
        envs: None,
        interfaces: None,
    };
    PolywrapClient::new(config)
}
