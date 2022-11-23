/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::{error::Error, invoke::Invoker};
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
            (get, $crate::wrap::module::ArgsGet),
            (post, $crate::wrap::module::ArgsPost),
        );
    };
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGet {
    pub url: String,
    pub request: Option<Request>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsPost {
    pub url: String,
    pub request: Option<Request>,
}

#[async_trait]
pub trait Module: PluginModule {
  async fn get(&mut self, args: &ArgsGet, invoker: Arc<dyn Invoker>) -> Result<Option<Response>, Error>;

  async fn post(&mut self, args: &ArgsPost, invoker: Arc<dyn Invoker>) -> Result<Option<Response>, Error>;
}
