use polywrap_client::client::PolywrapClient;
use polywrap_core::{client::ClientConfig, invoker::Invoker, package::WrapPackage, uri::Uri};
use polywrap_msgpack::msgpack;
use polywrap_resolvers::static_resolver::{StaticResolver, StaticResolverLike};
use serde_json::{from_value, json};
use std::{collections::HashMap, sync::Arc};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

use polywrap_plugin::{
    error::PluginError, implementor::plugin_impl, module::PluginModule, package::PluginPackage,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetEnvArgs {
    key: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Env {
    foo: String,
}

#[derive(Debug)]
pub struct PluginEnv {}

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
        abi: from_value::<WrapManifestAbi>(json!({})).unwrap(),
    }
}

#[test]
fn invoke_test() {
    let plugin = PluginEnv {};
    let package: PluginPackage = plugin.into();
    let module = Arc::new(package) as Arc<dyn WrapPackage>;

    let plugin_static_like =
        StaticResolverLike::Package(Uri::try_from("ens/env-plugin.eth").unwrap(), module);
    let static_resolver = StaticResolver::from(vec![plugin_static_like]);

    let env_val = msgpack!({"foo": "bar"});
    let envs = HashMap::from([(Uri::try_from("ens/env-plugin.eth").unwrap().uri, env_val)]);
    let client = PolywrapClient::new(ClientConfig {
        envs: Some(envs),
        interfaces: None,
        resolver: Arc::new(static_resolver),
    });

    let invoke_result = client
        .invoke::<bool>(
            &Uri::try_from("ens/env-plugin.eth").unwrap(),
            "checkEnvIsBar",
            Some(&msgpack!({"key": "foo"})),
            None,
            None,
        )
        .unwrap();

    assert!(invoke_result);
}
