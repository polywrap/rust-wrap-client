use std::sync::Arc;

use async_trait::async_trait;
use mapping::{parse_response, parse_request};
use polywrap_core::invoke::Invoker;
use polywrap_plugin::error::PluginError;
use wrap::{module::Module};
pub mod mapping;
pub mod wrap;

pub struct HttpPlugin {}

#[async_trait]
impl Module for HttpPlugin {
    async fn get(
        &mut self,
        args: &wrap::module::ArgsGet,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<wrap::types::Response>, PluginError> {
        let response = parse_request(&args.url, args.request.clone(), mapping::RequestMethod::GET).unwrap()
            .call()
            .map_err(|e| PluginError::ModuleError(e.to_string()))?;

        let parsed_response = parse_response(response)?;

        Ok(Some(parsed_response))
    }

    async fn post(
        &mut self,
        args: &wrap::module::ArgsPost,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<wrap::types::Response>, PluginError> {
        let response = parse_request(&args.url, args.request.clone(), mapping::RequestMethod::POST).unwrap()
            .call()
            .map_err(|e| PluginError::ModuleError(e.to_string()))?;

        let parsed_response = parse_response(response)?;

        Ok(Some(parsed_response))
    }
}

impl_traits!(HttpPlugin);
