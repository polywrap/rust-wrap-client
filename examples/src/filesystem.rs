extern crate polywrap_client;
extern crate polywrap_client_builder;
extern crate polywrap_client_default_config;
extern crate polywrap_core;
extern crate polywrap_fs_plugin;
extern crate polywrap_msgpack_serde;
extern crate polywrap_plugin;
extern crate serde;

use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_core::{client::ClientConfigBuilder, error::Error, macros::uri, uri::Uri};
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_msgpack_serde::{to_vec, bytes::ByteBuf};
use polywrap_plugin::package::PluginPackage;
use serde::Serialize;

#[derive(Serialize)]
struct WriteFileArgs {
    path: String,
    data: Vec<u8>,
}

#[derive(Serialize)]
struct FileArgs {
    path: String,
}

fn main() {
    let uri = uri!("wrapscan.io/polywrap/file-system@1.0");
    let mut config = PolywrapClientConfig::new();
    let fs_package = PluginPackage::from(FileSystemPlugin {});

    config.add_package(uri.clone(), Arc::new(fs_package));

    let file_path = "./fs-example.txt".to_string();
    let data = "Hello world!";

    let client = PolywrapClient::new(config.build());
    let write_file_result: Result<bool, Error> = client.invoke(
        &uri,
        "writeFile",
        Some(
            &to_vec(&WriteFileArgs {
                path: file_path.clone(),
                data: data.as_bytes().to_vec(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if write_file_result.is_err() {
        panic!(
            "Error writing file: {}",
            &write_file_result.unwrap_err().to_string()
        )
    }

    println!("File created!");

    let read_file_result: Result<ByteBuf, Error> = client.invoke(
        &uri,
        "readFile",
        Some(
            &to_vec(&FileArgs {
                path: file_path.clone(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if read_file_result.is_err() {
        panic!(
            "Error reading file: {}",
            &read_file_result.unwrap_err().to_string()
        )
    }

    println!(
        "Content file: {:#?}",
        String::from_utf8(read_file_result.unwrap().to_vec())
    );

    let remove_file_result: Result<bool, Error> = client.invoke(
        &uri,
        "rm",
        Some(&to_vec(&FileArgs { path: file_path }).unwrap()),
        None,
        None,
    );

    if remove_file_result.is_err() {
        panic!(
            "Error removing file: {}",
            &remove_file_result.unwrap_err().to_string()
        )
    }

    println!("File removed!");
}
