use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KnownNetwork {
    Mainnet = 1,
    Goerli = 5,
    BinanceSmartChain = 56,
    Sepolia = 11155111,
    CeloMainnet = 42220,
    CeloAlfajores = 44787,
    AvalancheMainnet = 43114,
    AvalancheFuji = 43113,
    PalmMainnet = 11297108109,
    PalmTestnet = 11297108099,
    AuroraMainnet = 1313161554,
    AuroraTestnet = 1313161555,
}

lazy_static! {
    static ref ALIASES: HashMap<String, KnownNetwork> = {
        let mut m = HashMap::new();
        m.insert("1".to_string(), KnownNetwork::Mainnet);
        m.insert("mainnet".to_string(), KnownNetwork::Mainnet);
        m.insert("5".to_string(), KnownNetwork::Goerli);
        m.insert("goerli".to_string(), KnownNetwork::Goerli);
        m.insert("11155111".to_string(), KnownNetwork::Sepolia);
        m.insert("sepolia".to_string(), KnownNetwork::Sepolia);
        m.insert("56".to_string(), KnownNetwork::BinanceSmartChain);
        m.insert("binance".to_string(), KnownNetwork::BinanceSmartChain);
        m.insert("42220".to_string(), KnownNetwork::CeloMainnet);
        m.insert("celo_mainnet".to_string(), KnownNetwork::CeloMainnet);
        m.insert("44787".to_string(), KnownNetwork::CeloAlfajores);
        m.insert("celo_alfajores".to_string(), KnownNetwork::CeloAlfajores);
        m.insert("43114".to_string(), KnownNetwork::AvalancheMainnet);
        m.insert(
            "avalanche_mainnet".to_string(),
            KnownNetwork::AvalancheMainnet,
        );
        m.insert("43113".to_string(), KnownNetwork::AvalancheFuji);
        m.insert("avalanche_fuji".to_string(), KnownNetwork::AvalancheFuji);
        m.insert("11297108109".to_string(), KnownNetwork::PalmMainnet);
        m.insert("palm_mainnet".to_string(), KnownNetwork::PalmMainnet);
        m.insert("11297108099".to_string(), KnownNetwork::PalmTestnet);
        m.insert("palm_testnet".to_string(), KnownNetwork::PalmTestnet);
        m.insert("1313161554".to_string(), KnownNetwork::AuroraMainnet);
        m.insert("aurora_mainnet".to_string(), KnownNetwork::AuroraMainnet);
        m.insert("1313161555".to_string(), KnownNetwork::AuroraTestnet);
        m.insert("aurora_testnet".to_string(), KnownNetwork::AuroraTestnet);
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

pub fn get_name(network: KnownNetwork) -> Option<&'static str> {
    NETWORK_NAMES.get(&network).cloned()
}
