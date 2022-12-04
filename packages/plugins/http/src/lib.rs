use std::sync::Arc;

use async_trait::async_trait;
use mapping::{parse_request, parse_response};
use polywrap_core::invoke::Invoker;
use polywrap_plugin::error::PluginError;
use wrap::{module::Module, types::ResponseType};
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
        let response = parse_request(&args.url, args.request.clone(), mapping::RequestMethod::GET)
            .unwrap()
            .send()
            .await
            .map_err(|e| PluginError::ModuleError(e.to_string()))?;

        let response_type = if let Some(r) = &args.request {
            r.response_type
        } else {
            ResponseType::TEXT
        };

        let parsed_response = parse_response(response, response_type).await?;

        Ok(Some(parsed_response))
    }

    async fn post(
        &mut self,
        args: &wrap::module::ArgsPost,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<wrap::types::Response>, PluginError> {
        let response = parse_request(
            &args.url,
            args.request.clone(),
            mapping::RequestMethod::POST,
        )
        .unwrap()
        .send()
        .await
        .map_err(|e| PluginError::ModuleError(e.to_string()))?;

        let response_type = if let Some(r) = &args.request {
            r.response_type
        } else {
            ResponseType::TEXT
        };

        let parsed_response = parse_response(response, response_type).await?;

        Ok(Some(parsed_response))
    }
}

impl_traits!(HttpPlugin);
