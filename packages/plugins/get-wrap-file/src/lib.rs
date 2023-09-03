use crate::wrap::wrap_info::get_manifest;
use std::sync::Arc;

use polywrap_plugin::*;
use wrap::module::{
    Module, ArgsGetFile,
};
pub mod wrap;

#[derive(Debug)]
pub struct GetWrapFilePlugin;

#[plugin_impl]
impl Module for GetWrapFilePlugin {
    fn get_file(
        &mut self,
        args: &ArgsGetFile,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<ByteBuf>, PluginError> {
        let uri: Uri = args.uri.parse().map_err(|e: ParseError| PluginError::InvocationError { exception: e.to_string() })?;

        let result = invoker.get_file(&uri, args.path.clone(), None).map_err(|e| PluginError::InvocationError { exception: e.to_string() })?;

        Ok(result.map(|x| ByteBuf::from(x)))
    }
}
