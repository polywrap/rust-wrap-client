use polywrap_client_builder::PolywrapClientConfig;
use polywrap_core::{client::ClientConfig, macros::uri, uri::Uri};
use polywrap_ethereum_wallet_plugin::{
    connection::Connection, connections::Connections, EthereumWalletPlugin,
};
use polywrap_plugin::package::PluginPackage;
use std::{collections::HashMap, sync::Arc};

pub struct Web3ClientConfig(PolywrapClientConfig);

impl Web3ClientConfig {
    fn get_ethereum_plugin() -> PluginPackage<EthereumWalletPlugin> {
        let mainnet_connection = Connection::new(
            "https://mainnet.infura.io/v3/f1f688077be642c190ac9b28769daecf".to_string(),
            None,
        )
        .unwrap();
        let goerli_connection = Connection::new(
            "https://goerli.infura.io/v3/f1f688077be642c190ac9b28769daecf".to_string(),
            None,
        )
        .unwrap();
        let connections = Connections::new(
            HashMap::from([
                ("mainnet".to_string(), mainnet_connection),
                ("goerli".to_string(), goerli_connection),
            ]),
            Some("mainnet".to_string()),
        );

        let wallet_plugin = EthereumWalletPlugin::new(connections);
        wallet_plugin.into()
    }
}

impl Default for Web3ClientConfig {
    fn default() -> Self {
        Self(PolywrapClientConfig {
            interfaces: Some(HashMap::from([(
                uri!("wrapscan.io/polywrap/uri-resolver@1.0"),
                vec![
                    uri!("wrapscan.io/polywrap/ens-text-record-uri-resolver@1.0"),
                    uri!("wrapscan.io/polywrap/ens-contenthash-uri-resolver@1.0"),
                    uri!("wrapscan.io/polywrap/ens-ipfs-contenthash-uri-resolver@1.0"),
                ],
            )])),
            packages: Some(vec![(
                uri!("plugin/ethereum-wallet@1.0"),
                Arc::new(Web3ClientConfig::get_ethereum_plugin()),
            )]),
            redirects: Some(HashMap::from([
                (
                    uri!("wrapscan.io/polywrap/ens-text-record-uri-resolver@1.0"),
                    uri!("wrap://ipfs/QmdYoDrXPxgjSoWuSWirWYxU5BLtpGVKd3z2GXKhW2VXLh"),
                ),
                (
                    uri!("wrapscan.io/polywrap/ethereum-wallet@1.0"),
                    uri!("plugin/ethereum-wallet@1.0"),
                ),
            ])),
            ..Default::default()
        })
    }
}

impl Into<PolywrapClientConfig> for Web3ClientConfig {
    fn into(self) -> PolywrapClientConfig {
        self.0
    }
}

impl Into<ClientConfig> for Web3ClientConfig {
    fn into(self) -> ClientConfig {
        self.0.into()
    }
}
