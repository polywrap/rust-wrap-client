use crate::networks::{get_name, KnownNetwork};
use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};
use std::fmt::Debug;
use thiserror::Error;
use tokio::runtime::Runtime;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum WalletError {
    #[error("Not signer given")]
    NoSignerFound,
    #[error("Wrong string format in signer")]
    WrongSignerGiven,
}

#[derive(Clone)]
pub struct Connection {
    pub provider: Provider<Http>,
    pub signer: Option<String>,
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection")
    }
}

impl Connection {
    pub fn new(provider: String, signer: Option<String>) -> Result<Self, String> {
        let provider = Provider::<Http>::try_from(provider);
        if let Err(e) = provider {
            return Err(format!("Error getting provider from network: {}", e));
        } else {
            Ok(Self {
                provider: provider.unwrap(),
                signer,
            })
        }
    }

    pub fn from_node(node: String, signer: Option<String>) -> Result<Self, String> {
        let connection = Connection::new(node, signer);
        if let Err(e) = connection {
            return Err(format!(
                "Error creating connection in from_node method: {}",
                e
            ));
        } else {
            connection
        }
    }

    pub fn from_network(network: KnownNetwork, signer: Option<String>) -> Result<Self, String> {
        let name = get_name(network);

        if name.is_none() {
            return Err(format!("Given network: {:#?} is not supported", network));
        };

        let name = name.unwrap();
        let connection = Connection::new(
            format!("https://{name}.infura.io/v3/1a8e6a8ab1df44ccb77d3e954082c5d4"),
            signer,
        );
        if let Err(e) = connection {
            return Err(format!(
                "Error creating connection in from_network method: {}",
                e
            ));
        } else {
            connection
        }
    }

    pub fn get_signer(&self) -> Result<LocalWallet, WalletError> {
        if let Some(s) = &self.signer {
            let wallet = s.parse::<LocalWallet>();
            if let Ok(w) = wallet {
                let chain_id = self.provider.get_chainid();
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let chain_id = Runtime::block_on(&runtime, chain_id).unwrap();
                Ok(w.with_chain_id(chain_id.as_u64()))
            } else {
                Err(WalletError::WrongSignerGiven)
            }
        } else {
            Err(WalletError::NoSignerFound)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Connection;

    fn create_connection() -> Connection {
        let provider = "https://goerli.infura.io/v3/41fbecf847994df5a9652b1210effd8a".to_string();
        let signer = Some(String::from(
            "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d",
        ));
        Connection::new(provider, signer).unwrap()
    }

    #[test]
    fn get_signer() {
        let connection = create_connection();
        let s = connection.get_signer();
        assert!(s.is_ok());
    }
}
