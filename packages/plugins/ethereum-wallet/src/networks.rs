use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KnownNetwork {
    Mainnet,
    Goerli,
    BinanceSmartChain,
    Sepolia,
    CeloMainnet,
    CeloAlfajores,
    AvalancheMainnet,
    AvalancheFuji,
    PalmMainnet,
    PalmTestnet,
    AuroraMainnet,
    AuroraTestnet,
}

impl KnownNetwork {
    fn chain_id(&self) -> &'static str {
        match self {
            Self::Mainnet => "1",
            Self::Goerli => "5",
            Self::BinanceSmartChain => "56",
            Self::Sepolia => "11155111",
            Self::CeloMainnet => "42220",
            Self::CeloAlfajores => "44787",
            Self::AvalancheMainnet => "43114",
            Self::AvalancheFuji => "43113",
            Self::PalmMainnet => "11297108109",
            Self::PalmTestnet => "11297108099",
            Self::AuroraMainnet => "1313161554",
            Self::AuroraTestnet => "1313161555",
        }
    }
}

lazy_static! {
    static ref ALIASES: HashMap<String, KnownNetwork> = {
        let mut m = HashMap::new();
        m.insert("mainnet".to_string(), KnownNetwork::Mainnet);
        m.insert(
            KnownNetwork::Mainnet.chain_id().to_string(),
            KnownNetwork::Mainnet
        );
        m.insert("goerli".to_string(), KnownNetwork::Goerli);
        m.insert(
            KnownNetwork::Goerli.chain_id().to_string(),
            KnownNetwork::Goerli
        );
        m.insert("sepolia".to_string(), KnownNetwork::Sepolia);
        m.insert(
            KnownNetwork::Sepolia.chain_id().to_string(),
            KnownNetwork::Sepolia
        );
        m.insert("binance".to_string(), KnownNetwork::BinanceSmartChain);
        m.insert(
            KnownNetwork::BinanceSmartChain.chain_id().to_string(),
            KnownNetwork::BinanceSmartChain,
        );
        m.insert("celo-mainnet".to_string(), KnownNetwork::CeloMainnet);
        m.insert(
            KnownNetwork::CeloMainnet.chain_id().to_string(),
            KnownNetwork::CeloMainnet,
        );
        m.insert("celo-alfajores".to_string(), KnownNetwork::CeloAlfajores);
        m.insert(
            KnownNetwork::CeloAlfajores.chain_id().to_string(),
            KnownNetwork::CeloAlfajores,
        );
        m.insert("avalanche-mainnet".to_string(), KnownNetwork::AvalancheMainnet);
        m.insert(
            KnownNetwork::AvalancheMainnet.chain_id().to_string(),
            KnownNetwork::AvalancheMainnet,
        );
        m.insert("avalanche-fuji".to_string(), KnownNetwork::AvalancheFuji);
        m.insert(
            KnownNetwork::AvalancheFuji.chain_id().to_string(),
            KnownNetwork::AvalancheFuji,
        );
        m.insert("palm-mainnet".to_string(), KnownNetwork::PalmMainnet);
        m.insert(
            KnownNetwork::PalmMainnet.chain_id().to_string(),
            KnownNetwork::PalmMainnet,
        );
        m.insert("palm-testnet".to_string(), KnownNetwork::PalmTestnet);
        m.insert(
            KnownNetwork::PalmTestnet.chain_id().to_string(),
            KnownNetwork::PalmTestnet,
        );
        m.insert("aurora-mainnet".to_string(), KnownNetwork::AuroraMainnet);
        m.insert(
            KnownNetwork::AuroraMainnet.chain_id().to_string(),
            KnownNetwork::AuroraMainnet,
        );
        m.insert("aurora-testnet".to_string(), KnownNetwork::AuroraTestnet);
        m.insert(
            KnownNetwork::AuroraTestnet.chain_id().to_string(),
            KnownNetwork::AuroraTestnet,
        );
        m
    };
    static ref NETWORK_NAMES: HashMap<KnownNetwork, &'static str> = {
        let mut m = HashMap::new();
        m.insert(KnownNetwork::Mainnet, "mainnet");
        m.insert(KnownNetwork::Goerli, "goerli");
        m.insert(KnownNetwork::Sepolia, "sepolia");
        m.insert(KnownNetwork::BinanceSmartChain, "binance");
        m.insert(KnownNetwork::CeloMainnet, "celo-mainnet");
        m.insert(KnownNetwork::CeloAlfajores, "celo-alfajores");
        m.insert(KnownNetwork::AvalancheMainnet, "avalanche-mainnet");
        m.insert(KnownNetwork::AvalancheFuji, "avalanche-fuji");
        m.insert(KnownNetwork::PalmMainnet, "palm-mainnet");
        m.insert(KnownNetwork::PalmTestnet, "palm-testnet");
        m.insert(KnownNetwork::AuroraMainnet, "aurora-mainnet");
        m.insert(KnownNetwork::AuroraTestnet, "aurora-testnet");
        m
    };
}

pub fn from_alias(alias: &str) -> Option<KnownNetwork> {
    ALIASES.get(&alias.to_lowercase()).cloned()
}

pub fn get_name(network: &KnownNetwork) -> Option<&'static str> {
    NETWORK_NAMES.get(network).cloned()
}