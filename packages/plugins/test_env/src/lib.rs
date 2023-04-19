use std::sync::Arc;

use polywrap_core::invoke::Invoker;
use polywrap_plugin::error::PluginError;
use polywrap_plugin_macro::plugin_impl;
use wrap::{
    module::{ArgsRequiredEnv, Module, ArgsOptEnv, ArgsNoEnv},
    types::Env,
};
use crate::wrap::wrap_info::get_manifest;
pub mod wrap;

#[derive(Debug)]
pub struct TestEnvPlugin {}

#[plugin_impl]
impl Module for TestEnvPlugin {
    fn required_env(
        &mut self,
        _: &ArgsRequiredEnv,
        _: Arc<dyn Invoker>,
        _: Env,
    ) -> Result<Option<bool>, PluginError> {
      unimplemented!()
    }

    fn opt_env(
        &mut self,
        _: &ArgsOptEnv,
        _: Arc<dyn Invoker>,
        _: Option<Env>,
    ) -> Result<Option<bool>, PluginError> {
      unimplemented!()
    }

    fn no_env(
        &mut self,
        _: &ArgsNoEnv,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
      unimplemented!()
    }
}
