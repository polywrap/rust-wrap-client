extern crate polywrap_client;
extern crate polywrap_client_builder;
extern crate polywrap_client_default_config;
extern crate polywrap_core;
extern crate polywrap_http_plugin;
extern crate polywrap_msgpack_serde;
extern crate polywrap_plugin;
extern crate serde;

use polywrap_client::{client::PolywrapClient, core::uri::Uri};
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::{client::ClientConfigBuilder, macros::uri};
use polywrap_msgpack_serde::{bytes::ByteBuf, to_vec};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AddFileResult {
    hash: String,
}

#[derive(Serialize, Deserialize)]
struct FileEntry {
    data: Vec<u8>,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct AddFileArgs {
    data: FileEntry,
    #[serde(rename = "ipfsProvider")]
    ipfs_provider: String,
}

#[derive(Serialize)]
struct CatArgs {
    cid: String,
    #[serde(rename = "ipfsProvider")]
    ipfs_provider: String,
}

fn main() {
    let ipfs_provider = "http://localhost:5001";
    let uri = uri!("wrapscan.io/polywrap/ipfs-http-client@1.0");

    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());

    let config = config.build();

    let client = PolywrapClient::new(config);

    let file_name = "hello-world.txt";
    let file_data = "Hello World!!!";

    println!("File Name: {}", file_name);
    println!("File Data: {}", file_data);

    let file_entry = FileEntry {
        data: file_data.as_bytes().to_vec(),
        name: file_name.to_string(),
    };

    let add_file_args = AddFileArgs {
        data: file_entry,
        ipfs_provider: ipfs_provider.to_string(),
    };

    let add_file_resp = client
        .invoke::<AddFileResult>(
            &uri.clone(),
            "addFile",
            Some(&to_vec(&add_file_args).unwrap()),
            None,
            None,
        )
        .unwrap();

    println!("Successfully Added: {}", add_file_resp.hash);

    let cat_resp = client
        .invoke::<ByteBuf>(
            &uri,
            "cat",
            Some(
                &to_vec(&CatArgs {
                    cid: add_file_resp.hash,
                    ipfs_provider: ipfs_provider.to_string(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();
    println!(
        "Cat Result: {}",
        String::from_utf8(cat_resp.to_vec()).unwrap()
    );
}
