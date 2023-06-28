/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, module::PluginModule};
use serde::{Serialize, Deserialize};
use serde_bytes::ByteBuf;
use super::types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsReadFile {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsReadFileAsString {
    pub path: String,
    pub encoding: Option<Encoding>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsExists {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsWriteFile {
    pub path: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsMkdir {
    pub path: String,
    pub recursive: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRm {
    pub path: String,
    pub recursive: Option<bool>,
    pub force: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRmdir {
    pub path: String,
}

pub trait Module: PluginModule {
  fn read_file(&mut self, args: &ArgsReadFile, invoker: Arc<dyn Invoker>) -> Result<ByteBuf, PluginError>;

  fn read_file_as_string(&mut self, args: &ArgsReadFileAsString, invoker: Arc<dyn Invoker>) -> Result<String, PluginError>;

  fn exists(&mut self, args: &ArgsExists, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;

  fn write_file(&mut self, args: &ArgsWriteFile, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;

  fn mkdir(&mut self, args: &ArgsMkdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;

  fn rm(&mut self, args: &ArgsRm, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;

  fn rmdir(&mut self, args: &ArgsRmdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;
}
