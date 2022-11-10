use std::{sync::Arc};

use polywrap_core::{invoke::Invoker, plugins::plugin_module::PluginModule};
use polywrap_manifest::versions::WrapManifest;

pub struct HttpPlugin {
    manifest: WrapManifest,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SomeFoo {
    foo: String,
    bar: u32,
}

impl HttpPlugin {
    pub fn new(manifest: WrapManifest) -> Self {
        let instance = Self { manifest };

        instance
    }

    pub fn get(
        &self,
        args: &SomeFoo,
        invoker: Arc<dyn Invoker>,
    ) -> Result<String, polywrap_core::error::Error> {
        Ok("get".to_string())
    }
}

impl PluginModule for HttpPlugin {
    fn get_manifest(
        &self,
    ) -> Result<polywrap_manifest::versions::WrapManifest, polywrap_core::error::Error> {
        Ok(self.manifest.clone())
    }

    fn _wrap_invoke(
        &self,
        method_name: &str,
        params: &serde_json::Value,
        invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<serde_json::Value, polywrap_core::error::Error> {
        match method_name {
            "get" => {
                let result = self.get(
                    &serde_json::from_value::<SomeFoo>(params.clone())
                        .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?,
                    invoker,
                )?;

                Ok(serde_json::to_value(result)
                    .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?)
            }
            _ => panic!("Method not found"),
        }
    }
}
