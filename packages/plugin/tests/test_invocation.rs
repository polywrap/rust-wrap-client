
use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{invoke::{Invoker, InvokeArgs}, env::{Env}, resolvers::{static_resolver::{StaticResolverLike, StaticResolver}, uri_resolution_context::UriPackage}, uri::Uri, client::ClientConfig};

use polywrap_manifest::versions::{WrapManifest, WrapManifestAbi};
use polywrap_msgpack::msgpack;
use polywrap_plugin::{error::PluginError, module::{PluginModule, PluginWithEnv}, package::PluginPackage, impl_plugin_traits};
use serde_json::{Value, from_value, json};
use futures::lock::Mutex;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetEnvArgs {
    key: String
}

pub struct PluginEnv {
    pub env: Env
}

#[async_trait]
pub trait Module: PluginModule {
  async fn check_env_is_bar(&mut self, args: &GetEnvArgs, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;
}


#[async_trait]
impl Module for PluginEnv {
    async fn check_env_is_bar(
        &mut self,
        args: &GetEnvArgs,
        _: Arc<dyn Invoker>
    ) -> Result<bool, PluginError> {
        let env = self.get_env(args.key.clone());
        if let Some(e) = env {
            return Ok(e.eq(&Value::String("bar".to_string())));
        }
        Ok(false)
    }
}


pub fn get_manifest() -> WrapManifest {
    WrapManifest {
        name: "env".to_string(),
        type_: "plugin".to_string(),
        version: "0.1".to_string(),
        abi: from_value::<WrapManifestAbi>(json!({})).unwrap()
    }
}

impl_plugin_traits!(
    PluginEnv, 
    get_manifest(),
    (check_env_is_bar, GetEnvArgs),
);

#[tokio::test]
async fn invoke_test() {
    
    let plugin = PluginEnv { env: Value::Null };
    let package: PluginPackage = plugin.into();
    let module = Arc::new(Mutex::new(package));

    let uri_package = UriPackage {
        package: module,
        uri: Uri::try_from("ens/env-plugin.eth").unwrap()
    };
    let plugin_static_like = StaticResolverLike::Package(uri_package);

    let static_resolver = StaticResolver::from(vec![
        plugin_static_like
    ]);

    let foo = json!({"foo": "bar"});
    let envs = HashMap::from([
        ( Uri::try_from("ens/env-plugin.eth").unwrap().uri, foo)
    ]);
    let client = PolywrapClient::new(
        ClientConfig {
            envs: Some(envs),
            interfaces: None,
            resolver: Arc::new(static_resolver),
        }
    );

    let invoke_args = InvokeArgs::Msgpack(msgpack!({"key": "foo"}));

    let invoke_result = client
        .invoke_and_decode::<bool>(
            &Uri::try_from("ens/env-plugin.eth").unwrap(),
            "checkEnvIsBar",
            Some(&invoke_args),
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(invoke_result, true);
}
