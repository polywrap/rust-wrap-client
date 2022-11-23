use std::sync::Arc;

use async_trait::async_trait;
use mapping::parse_response;
use polywrap_core::invoke::Invoker;
use polywrap_plugin::{module::PluginModule, package::PluginPackage, wrapper::PluginWrapper};
use tokio::sync::Mutex;
use wrap::{module::Module, wrap_info::get_manifest};
pub mod mapping;
pub mod wrap;

pub struct HttpPlugin {}

#[async_trait]
impl Module for HttpPlugin {
    async fn get(
        &mut self,
        args: &wrap::module::ArgsGet,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<wrap::types::Response>, polywrap_core::error::Error> {
        let response = ureq::get(&args.url)
            .call()
            .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?;

        let parsed_response = parse_response(response)?;

        Ok(Some(parsed_response))
    }

    async fn post(
        &mut self,
        args: &wrap::module::ArgsPost,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<wrap::types::Response>, polywrap_core::error::Error> {
        let response = ureq::get(&args.url)
            .call()
            .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?;

        let parsed_response = parse_response(response)?;

        Ok(Some(parsed_response))
    }
}

impl_traits!(HttpPlugin);
