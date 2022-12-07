
use std::{collections::HashMap, sync::Arc, hash::Hash};

use async_trait::async_trait;
use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{invoke::{Invoker, InvokeArgs}, env::{Env, Envs}, resolvers::{static_resolver::{StaticResolverLike, StaticResolver}, uri_resolution_context::UriPackage}, uri::Uri, client::ClientConfig};

use polywrap_manifest::versions::{WrapManifest, WrapManifestAbi};
use polywrap_msgpack::msgpack;
use polywrap_plugin::{error::PluginError, module::PluginModule, package::PluginPackage, impl_plugin_envs};
use serde_json::{Value, from_value, json};
use futures::lock::Mutex;

#[derive(serde::Serialize, serde::Deserialize)]
struct GetMapArgs {}

#[derive(serde::Serialize, serde::Deserialize)]
struct UpdateMapArgs {
    map: HashMap<String, u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct GetEnvArgs {
    key: String,
}
struct MockMapPlugin {
    map: HashMap<String, u32>,
    envs: Envs
}

impl MockMapPlugin {
    pub fn get_map(&self, _: GetMapArgs, _: Arc<dyn Invoker>) -> &HashMap<String, u32> {
        &self.map
    }

    pub fn update_map(
        &mut self,
        args: UpdateMapArgs,
        _: Arc<dyn Invoker>,
    ) -> &HashMap<String, u32> {
        for (arg_key, arg_value) in args.map.iter() {
            self.map.insert(
                arg_key.clone(),
                if let Some(existing_key) = self.map.get(arg_key) {
                    existing_key + arg_value
                } else {
                    *arg_value
                },
            );
        }

        &self.map
    }

    pub fn get_env(&self, args: GetEnvArgs, _: Arc<dyn Invoker>) -> Option<&Value> {
        self.env.get(args.key)
    }
}

impl_plugin_traits!(
    MockMapPlugin, 
    manifest, 
    (
        update_map, UpdateMapArgs,
    )
);

impl_plugin_envs!(MockMapPlugin);

#[tokio::test]
async fn invoke_test() {

    let mock = MockMapPlugin { map: HashMap::new(), envs: HashMap::new() };
    let package: PluginPackage = mock.into();
    let plugin = Arc::new(Mutex::new(package));
    let manifest = WrapManifest {
        name: "mockMap".to_string(),
        type_: "plugin".to_string(),
        version: "0.1".to_string(),
        abi: from_value::<WrapManifestAbi>(json!({})).unwrap()
    };


    let uri_package = UriPackage {
        package: plugin,
        uri: Uri::try_from("ens/mock-plugin.eth").unwrap()
    };
    let plugin_static_like = StaticResolverLike::Package(uri_package);
    let static_resolver = StaticResolver::from(vec![
        plugin_static_like
    ]);

    let bar = json!("bar");
    let envs = HashMap::from([
        ("foo".to_string(), bar)
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
        .invoke_and_decode::<Value>(
            &Uri::try_from("ens/mock-plugin.eth").unwrap(),
            "get_env",
            Some(&invoke_args),
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(invoke_result, json!("bar"));
}
