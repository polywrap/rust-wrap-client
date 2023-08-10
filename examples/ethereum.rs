use std::{collections::HashMap, sync::Arc};

use polywrap_client::client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::{client::ClientConfigBuilder, error::Error, macros::uri, uri::Uri};
use polywrap_ethereum_wallet_plugin::{
    connection::Connection, connections::Connections, EthereumWalletPlugin,
};
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::package::PluginPackage;
use serde::Serialize;

#[derive(Serialize)]
struct SignTypedDataArgs {
    payload: String,
}

fn main() {
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

    let result = client.invoke::<String>(
        &uri!("wrapscan.io/polywrap/ethers@1.0.0"),
        "signTypedData",
        Some(
            &to_vec(&SignTypedDataArgs {
                payload: "{\"primaryType\":\"Mail\",\"message\":{\"to\":{\"name\":\"Bob\",\"wallet\":\"0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB\"},\"contents\":\"Hello Bob.!\",\"from\":{\"name\":\"Cow\",\"wallet\":\"0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826\"}},\"types\":{\"Person\":[{\"type\":\"string\",\"name\":\"name\"},{\"type\":\"address\",\"name\":\"wallet\"}],\"Mail\":[{\"type\":\"Person\",\"name\":\"from\"},{\"type\":\"Person\",\"name\":\"to\"},{\"type\":\"string\",\"name\":\"contents\"}],\"EIP712Domain\":[{\"type\":\"string\",\"name\":\"name\"},{\"type\":\"string\",\"name\":\"version\"},{\"type\":\"uint256\",\"name\":\"chainId\"},{\"type\":\"address\",\"name\":\"verifyingContract\"}]},\"domain\":{\"chainId\":1,\"name\":\"Ether Mail\",\"verifyingContract\":\"0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC\",\"version\":\"1\"}}".to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    println!("{:#?}", result);
}
