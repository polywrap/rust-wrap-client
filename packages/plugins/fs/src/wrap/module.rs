/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoke::Invoker;
use polywrap_plugin::{error::PluginError};
use polywrap_plugin::module::PluginModule;
use serde::{Serialize, Deserialize};
use super::types::*;
use async_trait::async_trait;
pub use polywrap_plugin::impl_plugin_traits;

#[macro_export]
macro_rules! impl_traits {
    ($plugin_type:ty) => {
        $crate::wrap::module::impl_plugin_traits!(
            $plugin_type,
            $crate::wrap::wrap_info::get_manifest(),
            (read_file, $crate::wrap::module::ArgsReadFile),
            (read_file_as_string, $crate::wrap::module::ArgsReadFileAsString),
            (exists, $crate::wrap::module::ArgsExists),
            (write_file, $crate::wrap::module::ArgsWriteFile),
            (mkdir, $crate::wrap::module::ArgsMkdir),
            (rm, $crate::wrap::module::ArgsRm),
            (rmdir, $crate::wrap::module::ArgsRmdir),
        );
    };
}

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

#[async_trait]
pub trait Module: PluginModule {
  async fn read_file(&mut self, args: &ArgsReadFile, invoker: Arc<dyn Invoker>) -> Result<Vec<u8>, PluginError>;

  async fn read_file_as_string(&mut self, args: &ArgsReadFileAsString, invoker: Arc<dyn Invoker>) -> Result<String, PluginError>;

  async fn exists(&mut self, args: &ArgsExists, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;

  async fn write_file(&mut self, args: &ArgsWriteFile, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;

  async fn mkdir(&mut self, args: &ArgsMkdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;

  async fn rm(&mut self, args: &ArgsRm, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;

  async fn rmdir(&mut self, args: &ArgsRmdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;
}
