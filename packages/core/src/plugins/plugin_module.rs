use std::{sync::Arc};

use polywrap_manifest::versions::WrapManifest;
use serde_json::Value;

use crate::error::Error;

pub trait PluginModule: Send + Sync {
    fn get_manifest(&self) -> Result<WrapManifest, Error>;
    fn _wrap_invoke(
        &self,
        method_name: &str,
        params: &Value,
        invoker: Arc<dyn crate::invoke::Invoker>,
    ) -> Result<Value, Error>;
}
