use polywrap_client::{
    client::Client,
    resolvers::static_resolver::{StaticResolver, StaticResolverLike},
};
use polywrap_ethereum_wallet_plugin::{
    connection::Connection, connections::Connections, EthereumWalletPlugin,
};
use polywrap_plugin::*;
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

pub mod request;

#[derive(Serialize)]
pub struct ConnectionArgs {
    #[serde(rename = "networkNameOrChainId")]
    network_name_or_chain_id: Option<String>,
    node: Option<String>,
}

fn get_client() -> Client {
    let bsc_connection = Connection::new(
        "https://bsc-dataseed1.binance.org/".to_string(),
        Some(String::from(
            "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d",
        )),
    )
    .unwrap();
    // let localhost_connection = Connection::new(
    //     "http://localhost:8545".to_string(),
    //     Some(String::from(
    //         "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    //     )),
    // )
    // .unwrap();
    let connections = Connections::new(
        HashMap::from([
            ("bsc".to_string(), bsc_connection),
            // (
            //     "testnet".to_string(),
            //     localhost_connection
            // )
        ]),
        Some("bsc".to_string()),
    );

    let wallet_plugin = EthereumWalletPlugin::new(connections);
    let plugin_pkg: PluginPackage<EthereumWalletPlugin> = wallet_plugin.into();
    let package = Arc::new(plugin_pkg);

    let resolver = StaticResolver::from(vec![StaticResolverLike::Package(
        Uri::try_from("plugin/ethereum-wallet").unwrap(),
        package,
    )]);

    Client::new(ClientConfig {
        resolver: Arc::new(resolver),
        interfaces: None,
        envs: None,
    })
}

#[derive(Serialize)]
pub struct ArgsSignerAddress {
    pub connection: Option<ConnectionArgs>,
}

#[test]
fn get_signer_address() {
    let client = get_client();
    let response = client.invoke::<String>(
        &Uri::try_from("plugin/ethereum-wallet").unwrap(),
        "signerAddress",
        Some(
            &to_vec(&ArgsSignerAddress {
                connection: Some(ConnectionArgs {
                    network_name_or_chain_id: Some("bsc".to_string()),
                    node: None,
                }),
            })
            .unwrap(),
        ),
        None,
        None,
    );
    assert_eq!(
        response.unwrap(),
        "0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1".to_string()
    )
}

#[derive(Serialize)]
struct SignMessageArgs {
    message: ByteBuf,
}

#[test]
fn sign_message() {
    let client = get_client();
    let response = client.invoke::<String>(
        &Uri::try_from("plugin/ethereum-wallet").unwrap(),
        "signMessage",
        Some(
            &to_vec(&SignMessageArgs {
                message: ByteBuf::from("Hello World".as_bytes()),
            })
            .unwrap(),
        ),
        None,
        None,
    );
    assert_eq!(
        response.unwrap(),
        "a4708243bf782c6769ed04d83e7192dbcf4fc131aa54fde9d889d8633ae39dab03d7babd2392982dff6bc20177f7d887e27e50848c851320ee89c6c63d18ca761c"
    )
}
