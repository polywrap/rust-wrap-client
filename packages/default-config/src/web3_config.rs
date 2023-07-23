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
            interfaces: Some(HashMap::from([
                (
                    "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
                    vec![
                        uri!("ens/wraps.eth:ens-text-record-uri-resolver-ext@1.0.2"),
                        uri!("ens/wraps.eth:ens-uri-resolver-ext@1.0.2"),
                        uri!("ens/wraps.eth:ens-ipfs-contenthash-uri-resolver-ext@1.0.1"),
                    ],
                ),
            ])),
            packages: Some(vec![(
                uri!("wrap://ens/wraps.eth:ethereum-provider@2.0.0"),
                Arc::new(Web3ClientConfig::get_ethereum_plugin()),
            )]),
            redirects: Some(HashMap::from([
                (
                    uri!("wrap://ens/wraps.eth:ens@0.1.0"),
                    uri!("wrap://ipfs/QmQS8cr21euKYW7hWAhiSYXgvdcAtbPbynKqRW2CzAJPYe"),
                ),
                (
                    uri!("ens/wraps.eth:ens-text-record-uri-resolver-ext@1.0.2"),
                    uri!("wrap://ipfs/Qmaqs7rmoW4AKtmfmBHrWw9iRNY8Bg78fcS1hpqB7R9gev"),
                ),
                (
                    uri!("ens/wraps.eth:ens-uri-resolver-ext@1.0.2"),
                    uri!("wrap://ipfs/QmV4S2BBwawQTxKCTCvjRuWt8EHkicZ3oM3S2B5JziAcrA"),
                ),
                (
                    uri!("ens/wraps.eth:ens-ipfs-contenthash-uri-resolver-ext@1.0.2"),
                    uri!("wrap://ipfs/QmT54TKaQmNktg2eUVMUjWbjVDBSpapZvnFdkDrjejLebE"),
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
