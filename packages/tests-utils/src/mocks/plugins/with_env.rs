use polywrap_core::invoker::Invoker;
use polywrap_plugin::error::PluginError;
use polywrap_plugin::{implementor::plugin_impl, module::PluginModule, JSON};
use serde::{Deserialize, Serialize};

use std::{fmt::Debug, sync::Arc};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetEnvArgs {
    key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Env {
    foo: String,
}

#[derive(Debug)]
pub struct PluginEnv;

pub trait Module: PluginModule {
    fn check_env_is_bar(
        &mut self,
        args: &GetEnvArgs,
        invoker: Arc<dyn Invoker>,
        env: Option<Env>,
    ) -> Result<bool, PluginError>;
}

#[plugin_impl]
impl Module for PluginEnv {
    fn check_env_is_bar(
        &mut self,
        args: &GetEnvArgs,
        _: Arc<dyn Invoker>,
        _env: Option<Env>,
    ) -> Result<bool, PluginError> {
        if let Some(_env) = _env {
            let value = match args.key.as_str() {
                "foo" => &_env.foo,
                &_ => panic!("Property does not exist"),
            };
            return Ok(value == "bar");
        }

        Ok(false)
    }
}

pub fn get_manifest() -> WrapManifest {
    WrapManifest {
        name: "env".to_string(),
        type_: "plugin".to_string(),
        version: "0.1".to_string(),
        abi: JSON::from_value::<WrapManifestAbi>(JSON::json!({})).unwrap(),
    }
}
