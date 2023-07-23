/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, module::PluginModule};
use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRequest {
    pub method: String,
    pub params: Option<String>,
    pub connection: Option<Connection>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsWaitForTransaction {
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub confirmations: u32,
    pub timeout: Option<u32>,
    pub connection: Option<Connection>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSignerAddress {
    pub connection: Option<Connection>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSignMessage {
    #[serde(with = "serde_bytes")]
    pub message: Vec<u8>,
    pub connection: Option<Connection>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSignTransaction {
    #[serde(with = "serde_bytes")]
    pub rlp: Vec<u8>,
    pub connection: Option<Connection>,
}

pub trait Module: PluginModule {
  fn request(&mut self, args: &ArgsRequest, invoker: Arc<dyn Invoker>) -> Result<String, PluginError>;

  fn wait_for_transaction(&mut self, args: &ArgsWaitForTransaction, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;

  fn signer_address(&mut self, args: &ArgsSignerAddress, invoker: Arc<dyn Invoker>) -> Result<Option<String>, PluginError>;

  fn sign_message(&mut self, args: &ArgsSignMessage, invoker: Arc<dyn Invoker>) -> Result<String, PluginError>;

  fn sign_transaction(&mut self, args: &ArgsSignTransaction, invoker: Arc<dyn Invoker>) -> Result<String, PluginError>;
}
