
use polywrap_core::{invoke::{Invoker}, env::Env};

use polywrap_plugin::{
    error::PluginError, module::PluginModule,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetEnvArgs {
    key: String,
}

#[derive(Debug)]
pub struct PluginEnv {}

pub trait Module: PluginModule {
    fn check_env_is_bar(
        &mut self,
        args: &GetEnvArgs,
        invoker: &dyn Invoker,
        env: Option<&Env>,
    ) -> Result<bool, PluginError>;
}

// #[plugin_impl]
// impl Module for PluginEnv {
//     fn check_env_is_bar(
//         &mut self,
//         args: &GetEnvArgs,
//         _: &dyn Invoker,
//         env: Option<&Env>,
//     ) -> Result<bool, PluginError> {
//         if let Some(env) = env {
//             if let Some(value) = env.get(args.key.clone()) {
//                 return Ok(value.eq(&Value::String("bar".to_string())));
//             }
//         }

//         Ok(false)
//     }
// }

// pub fn get_manifest() -> WrapManifest {
//     WrapManifest {
//         name: "env".to_string(),
//         type_: "plugin".to_string(),
//         version: "0.1".to_string(),
//         abi: from_value::<WrapManifestAbi>(json!({})).unwrap(),
//     }
// }

// #[test]
// fn invoke_test() {
//     let plugin = PluginEnv {};
//     let package: PluginPackage = plugin.into();
//     let module = Arc::new(package) as Arc<dyn WrapPackage>;

//     let plugin_static_like = StaticResolverLike::Package(Uri::try_from("ens/env-plugin.eth").unwrap(), module);
//     let static_resolver = StaticResolver::from(vec![
//         plugin_static_like
//     ]);

//     let foo = json!({"foo": "bar"});
//     let envs = HashMap::from([(Uri::try_from("ens/env-plugin.eth").unwrap().uri, foo)]);
//     let client = PolywrapClient::new(ClientConfig {
//         envs: Some(envs),
//         interfaces: None,
//         resolver: Arc::new(static_resolver),
//     });

//     let invoke_result = client
//         .invoke::<bool>(
//             &Uri::try_from("ens/env-plugin.eth").unwrap(),
//             "checkEnvIsBar",
//             Some(&msgpack!({"key": "foo"})),
//             None,
//             None,
//         )
//         .unwrap();

//     assert!(invoke_result);
// }
