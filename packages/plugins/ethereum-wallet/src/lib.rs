use crate::wrap::wrap_info::get_manifest;
use connection::WalletError;
use connections::Connections;
use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Middleware, PendingTransaction, Provider},
    signers::Signer,
    types::{
        transaction::{eip2718::TypedTransaction, eip712::TypedData},
        TransactionRequest, TxHash,
    },
};
use polywrap_plugin::*;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use tokio::runtime::Runtime;
use wrap::module::{
    ArgsRequest, ArgsSignMessage, ArgsSignTransaction, ArgsSignerAddress, ArgsWaitForTransaction,
    Module,
};
use std::result::Result;

pub mod connection;
pub mod connections;
mod networks;
mod types;
mod wrap;

pub use types::*;

#[derive(Debug)]
pub struct EthereumWalletPlugin {
    connections: Connections,
}

struct Params;

impl Params {
    fn sanatize(method: &str, params: &Option<JSONString>) -> Vec<Value> {
        if Params::is_transaction_method(method) {
            if let Some(params) = params {
                match method {
                    "eth_call" => Params::parse::<EthCallParamaterTypes>(params),
                    _ => Params::parse::<TypedTransaction>(params),
                }
            } else {
                panic!("Method {method} needs params")
            }
        } else if let Some(params) = params {
            match method {
                "eth_getBlockByNumber" => Params::parse::<GetBlockByNumberParamaterTypes>(params),
                "eth_feeHistory" => Params::parse::<FeeHistoryArgs>(params),
                "eth_signTypedData_v4" => Params::parse::<SignTypedDataArgs>(params),
                _ => {
                    let p = from_str(params.to_json().to_string().as_str());
                    if p.is_err() {
                        vec![]
                    } else {
                        p.unwrap()
                    }
                }
            }
        } else {
            vec![]
        }
    }

    fn parse<T: Serialize + for<'a> Deserialize<'a> + std::fmt::Debug>(
        values: &JSONString,
    ) -> Vec<Value> {
        let params_value = from_str::<Vec<T>>(values.to_json().to_string().as_str());

        if let Ok(v) = params_value {
            v.iter()
                .map(|value| to_value(value).unwrap())
                .collect::<Vec<Value>>()
        } else {
            let err = params_value.unwrap_err();
            panic!("Error parsing paremeters: {}", err)
        }
    }

    fn is_transaction_method(method: &str) -> bool {
        let transaction_methods = ["eth_sendTransaction", "eth_estimateGas", "eth_call"];
        transaction_methods.contains(&method)
    }
}

impl EthereumWalletPlugin {
    pub fn new(connections: Connections) -> Self {
        Self { connections }
    }
}

#[plugin_impl]
impl Module for EthereumWalletPlugin {
    fn request(
        &mut self,
        args: &ArgsRequest,
        _: Arc<dyn Invoker>,
    ) -> Result<JSONString, PluginError> {
        let connection = self.connections.get_connection(args.connection.clone());
        let provider: &Provider<Http> = &connection.provider;
        let method = args.method.as_str();
        let parameters = Params::sanatize(method, &args.params);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        match method {
            "eth_signTypedData_v4" => {
                let signer = connection.get_signer().unwrap();
                let typed_data: TypedData = from_value(parameters[1].clone()).unwrap();
                let hash = Runtime::block_on(&runtime, signer.sign_typed_data(&typed_data));
                let hash = format!("0x{}", hash.unwrap().to_string());
                Ok(JSONString::new(Value::String(hash)))
            }
            "eth_sendTransaction" => {
                let signer = connection.get_signer().unwrap();
                let tx: TransactionRequest = from_value(parameters[0].clone()).unwrap();
                let client = SignerMiddleware::new(provider, signer);
                let hash = Runtime::block_on(&runtime, client.send_transaction(tx, None));
                Ok(JSONString::new(Value::String(format!(
                    "{:#?}",
                    hash.unwrap().tx_hash()
                ))))
            }
            _ => {
                let response = Runtime::block_on(
                    &runtime,
                    provider.request::<Vec<Value>, Value>(method, parameters),
                );

                let result = response.map_err(|e| e.to_string()).unwrap();

                match result {
                    Value::String(r) => Ok(JSONString::new(Value::String(r))),
                    Value::Object(object) => Ok(JSONString::new(Value::Object(object))),
                    _ => Ok(JSONString::new(Value::String("".to_string()))),
                }
            }
        }
    }

    fn wait_for_transaction(
        &mut self,
        args: &ArgsWaitForTransaction,
        _: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        let connection = self.connections.get_connection(args.connection.clone());
        let pending_transaction = PendingTransaction::new(
            TxHash::from_str(&args.tx_hash).unwrap(),
            &connection.provider,
        );

        // pending_transaction.confirmations(args.confirmations.try_into().unwrap());
        // if let Some(t) = args.timeout {
        //     let duration = Duration::new(0, t);
        //     pending_transaction.interval(duration);
        // };

        let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();
        let tx = Runtime::block_on(&runtime, pending_transaction);

        Ok(matches!(tx, Ok(Some(_))))
    }

    fn signer_address(
        &mut self,
        args: &ArgsSignerAddress,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<String>, PluginError> {
        let connection = self.connections.get_connection(args.connection.clone());
        let signer = connection.get_signer();
        match signer {
            Ok(s) => Ok(Some(format!("0x{:x}", s.address()))),
            Err(e) => {
                if let WalletError::WrongSignerGiven = e {
                    Err(PluginError::InvocationError {
                        exception: "Signer private key not valid".to_string(),
                    })
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn sign_message(
        &mut self,
        args: &ArgsSignMessage,
        _: Arc<dyn Invoker>,
    ) -> Result<String, PluginError> {
        let connection = self.connections.get_connection(args.connection.clone());
        let signer = connection.get_signer();
        match signer {
            Ok(s) => {
                let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();
                let response = Runtime::block_on(&runtime, s.sign_message(args.message.to_vec()));
                if let Ok(signature) = response {
                    Ok(format!("{:#}", signature))
                } else {
                    Err(PluginError::InvocationError {
                        exception: "Error in sign message method".to_string(),
                    })
                }
            }
            Err(_) => Err(PluginError::InvocationError {
                exception: "Signer no available".to_string(),
            }),
        }
    }

    fn sign_transaction(
        &mut self,
        args: &ArgsSignTransaction,
        _: Arc<dyn Invoker>,
    ) -> Result<String, PluginError> {
        let connection = self.connections.get_connection(args.connection.clone());
        let signer = connection.get_signer();
        match signer {
            Ok(s) => {
                let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();
                let tx: TypedTransaction =
                    from_value(to_value(args.rlp.to_vec()).unwrap()).unwrap();

                let response = Runtime::block_on(&runtime, s.sign_transaction(&tx));
                if let Ok(signature) = response {
                    Ok(format!("{:#}", signature))
                } else {
                    Err(PluginError::InvocationError {
                        exception: "Error in sign transaction method".to_string(),
                    })
                }
            }
            Err(_) => Err(PluginError::InvocationError {
                exception: "Signer no available".to_string(),
            }),
        }
    }
}
