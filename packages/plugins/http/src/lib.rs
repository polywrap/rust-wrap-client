use std::sync::Arc;

use mapping::parse_response;
use polywrap_core::invoke::Invoker;
use wrap::module::Module;
pub mod mapping;
pub mod wrap;

pub struct HttpPlugin {}

impl Module for HttpPlugin {
    fn get(
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

    fn post(
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

impl_plugin_module!(HttpPlugin);
