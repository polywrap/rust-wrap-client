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
use polywrap_msgpack_serde::{to_vec, JSON::json};
use polywrap_plugin::package::PluginPackage;
use serde::Serialize;

#[derive(Serialize)]
struct SignTypedDataArgs {
    payload: String,
}

#[derive(Serialize)]
struct GetBalanceArgs {
    address: String,
}

#[derive(Serialize)]
struct ToEthArgs {
    wei: String,
}

fn main() {
    let ethers_core_uri = uri!("wrapscan.io/polywrap/ethers@1.0.0");
    let ethers_util_uri = uri!("wrapscan.io/polywrap/ethers-utils@1.0.0");
    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());

    let mainnet_connection = Connection::new(
        "https://mainnet.infura.io/v3/f1f688077be642c190ac9b28769daecf".to_string(),
        Some(String::from(
            "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d",
        )),
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

    let balance = client.invoke::<String>(
        &ethers_core_uri,
        "getBalance",
        Some(
            &to_vec(&GetBalanceArgs {
                address: "0x00000000219ab540356cbb839cbe05303d7705fa".to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if balance.is_err() {
        panic!(
            "Error with get balance: {}",
            &balance.unwrap_err().to_string()
        )
    }

    println!("Balance in Wei: {}", balance.clone().unwrap());

    let balance_in_eth = client.invoke::<String>(
        &ethers_util_uri,
        "toEth",
        Some(
            &to_vec(&ToEthArgs {
                wei: balance.unwrap(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if balance_in_eth.is_err() {
        panic!(
            "Error with get balance: {}",
            &balance_in_eth.unwrap_err().to_string()
        )
    }

    println!("Balance in Eth: {}", balance_in_eth.clone().unwrap());

    let domain = json!({
        "name": "Ether Mail",
        "version": "1",
        "chainId": 1,
        "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC",
    });
    let message = json!({
        "from": {
            "name": "Cow",
            "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
        },
        "to": {
            "name": "Bob",
            "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
        },
        "contents": "Hello, Bob!"
    });

    let types = json!({
        "EIP712Domain": [
            {
                "type": "string",
                "name": "name"
            },
            {
                "type": "string",
                "name": "version",
            },
            {
                "type": "uint256",
                "name": "chainId",
            },
            {
                "type": "address",
                "name": "verifyingContract",
            },
        ],
        "Person": [
            { "name": "name", "type": "string" },
            { "name": "wallet", "type": "address" }
        ],
        "Mail": [
            { "name": "from", "type": "Person" },
            { "name": "to", "type": "Person" },
            { "name": "contents", "type": "string" },
        ]
    });
    let payload = json!({
        "domain": domain,
        "types": types,
        "primaryType": "Mail",
        "message": message,
    });
    let sign_typed_data_result = client.invoke::<String>(
        &ethers_core_uri,
        "signTypedData",
        Some(
            &to_vec(&SignTypedDataArgs {
                payload: payload.to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if sign_typed_data_result.is_err() {
        panic!(
            "Error with sign typed data: {}",
            &sign_typed_data_result.unwrap_err().to_string()
        )
    }

    println!("Signed typed data: {}", sign_typed_data_result.unwrap());
}
