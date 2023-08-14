extern crate polywrap_client;
extern crate polywrap_client_builder;
extern crate polywrap_client_default_config;
extern crate polywrap_core;
extern crate polywrap_ethereum_wallet_plugin;
extern crate polywrap_msgpack_serde;
extern crate polywrap_plugin;
extern crate serde;

use std::{collections::HashMap, sync::Arc};

use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::{client::ClientConfigBuilder, macros::uri, uri::Uri};
use polywrap_ethereum_wallet_plugin::{
    connection::Connection, connections::Connections, EthereumWalletPlugin,
};
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::package::PluginPackage;
use serde::Serialize;

#[derive(Serialize)]
struct GetContentHashArgs {
    #[serde(rename = "resolverAddress")]
    resolver_address: String,
    domain: String,
}

#[derive(Serialize)]
struct GetResolverArgs {
    #[serde(rename = "registryAddress")]
    registry_address: String,
    domain: String,
}

fn main() {
    let domain = "vitalik.eth".to_string();
    let ens_uri = uri!("wrapscan.io/polywrap/ens@1.0.0");
    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());

    let mainnet_connection = Connection::new(
        "https://mainnet.infura.io/v3/f1f688077be642c190ac9b28769daecf".to_string(),
        None,
    )
    .unwrap();

    let connections = Connections::new(
        HashMap::from([("mainnet".to_string(), mainnet_connection)]),
        Some("mainnet".to_string()),
    );

    let wallet_plugin = EthereumWalletPlugin::new(connections);
    let plugin_pkg: PluginPackage<EthereumWalletPlugin> = wallet_plugin.into();
    let package = Arc::new(plugin_pkg);

    config.add_package(uri!("wrapscan.io/polywrap/ethereum-wallet@1.0"), package);

    let client = PolywrapClient::new(config.build());

    let resolver_address = client.invoke::<String>(
        &ens_uri,
        "getResolver",
        Some(
            &to_vec(&GetResolverArgs {
                registry_address: "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e".to_string(),
                domain: domain.clone(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if resolver_address.is_err() {
        panic!(
            "Error with get resolver: {}",
            &resolver_address.unwrap_err().to_string()
        )
    }

    println!("Resolver address: {}", resolver_address.clone().unwrap());

    let content_hash = client.invoke::<String>(
        &ens_uri,
        "getContentHash",
        Some(
            &to_vec(&GetContentHashArgs {
                resolver_address: resolver_address.unwrap(),
                domain: domain.clone(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if content_hash.is_err() {
        panic!(
            "Error with get content hash: {}",
            &content_hash.unwrap_err().to_string()
        )
    }
    println!(
        "Content hash of {}: {}",
        domain,
        content_hash.clone().unwrap()
    );
}
