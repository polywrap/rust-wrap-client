use std::{collections::HashMap, sync::Arc};

use polywrap_client_builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler};
use polywrap_core::{
    uri::Uri,
    client::UriRedirect,
    wrapper::{Wrapper, GetFileOptions},
    invoke::{Invoker, InvokeArgs},
    env::Env,
    uri_resolution_context::UriResolutionContext,
    error::Error, package::WrapPackage
};
use serde_json::json;
use async_trait::async_trait;
// use tokio::sync::Mutex;

#[test]
fn test_env_methods() {
    let mut builder = BuilderConfig::new(None);
    let uri = Uri::new("wrap://ens/wrapper.eth");

    assert_eq!(builder.envs.is_none(), true);

    builder.add_env(uri.clone(), json!({ "d": "d" }));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.clone().uri);

    assert_eq!(env_from_builder.is_some(), true);
    assert_eq!(env_from_builder.unwrap(), &json!({ "d": "d" }));

    let mut envs = HashMap::new();
    envs.insert(uri.clone().uri, json!({"a": "a", "b": "b"}));

    builder.add_envs(envs);

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.clone().uri);
    assert_eq!(env_from_builder.unwrap(), &json!({ "d": "d", "a": "a", "b": "b" }));

    builder.set_env(uri.clone(), json!({"c": "c"}));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.clone().uri);
    assert_eq!(env_from_builder.unwrap(), &json!({ "c": "c" }));

    builder.remove_env(uri.clone());

    assert_eq!(builder.envs.is_none(), true);
}

#[test]
fn test_interface_implementation_methods() {
    let mut builder = BuilderConfig::new(None);

    let interface_uri = Uri::new("wrap://ens/interface.eth");
    let implementation_a_uri = Uri::new("wrap://ens/implementation-a.eth");
    let implementation_b_uri = Uri::new("wrap://ens/implementation-b.eth");

    assert_eq!(builder.interfaces.is_none(), true);

    builder.add_interface_implementations(
        interface_uri.clone(), 
        vec![implementation_a_uri.clone(), implementation_b_uri.clone()]
    );

    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.clone().uri).unwrap();
    assert_eq!(builder.interfaces.is_some(), true);
    assert_eq!(implementations, &vec![implementation_a_uri.clone(), implementation_b_uri.clone()]);

    let implementation_c_uri = Uri::new("wrap://ens/implementation-c.eth");
    builder.add_interface_implementation(interface_uri.clone(), implementation_c_uri.clone());

    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.clone().uri).unwrap();
    assert_eq!(implementations, &vec![
        implementation_a_uri.clone(), 
        implementation_b_uri.clone(),
        implementation_c_uri.clone()
    ]);

    builder.remove_interface_implementation(interface_uri.clone(), implementation_b_uri.clone());
    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.clone().uri).unwrap();
    assert_eq!(implementations, &vec![
        implementation_a_uri.clone(),
        implementation_c_uri.clone()
    ]);

}

#[test]
fn test_redirects() {
    let mut builder = BuilderConfig::new(None);
    assert_eq!(builder.redirects.is_some(), false);

    let redirects = vec![
        UriRedirect{
            from: Uri::from_string("ens/c.eth").unwrap(), 
            to: Uri::from_string("ens/d.eth").unwrap()
        },
        UriRedirect{
            from: Uri::from_string("ens/f.eth").unwrap(), 
            to: Uri::from_string("ens/g.eth").unwrap()
        },
    ];
    builder.add_redirects(redirects);

    assert_eq!(builder.redirects.is_some(), true);
    let builder_redirects = builder.redirects.unwrap();
    assert_eq!(builder_redirects[0].from, Uri::from_string("ens/c.eth").unwrap());
    assert_eq!(builder_redirects[0].to, Uri::from_string("ens/d.eth").unwrap());
    assert_eq!(builder_redirects[1].from, Uri::from_string("ens/f.eth").unwrap());
    assert_eq!(builder_redirects[1].to, Uri::from_string("ens/g.eth").unwrap());

    let mut builder = BuilderConfig::new(None);
    assert_eq!(builder.redirects.is_some(), false);

    builder.add_redirect(Uri::from_string("ens/a.eth").unwrap(), Uri::from_string("ens/b.eth").unwrap());
    assert_eq!(builder.redirects.is_some(), true);

    builder.remove_redirect(Uri::from_string("ens/a.eth").unwrap());
    assert_eq!(builder.redirects.is_some(), false);
}

#[test]
fn test_resolvers() {

}
#[actix_rt::test]
async fn test_wrappers() {
    struct MockWrapper;
    // #[async_trait]
    // impl Wrapper for MockWrapper {
    //     async fn invoke(
    //         &mut self,
    //         invoker: Arc<dyn Invoker>,
    //         uri: &Uri,
    //         method: &str,
    //         args: Option<&InvokeArgs>,
    //         env: Option<Env>,
    //         resolution_context: Option<&mut UriResolutionContext>,
    //     ) -> Result<Vec<u8>, Error> {
    //         Ok(vec![])
    //     }
    //     fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error> {
    //         Ok(vec![])
    //     }
    // }

    // struct MockPackage;
    // #[async_trait]
    // impl WrapPackage for MockPackage {
    //     async fn create_wrapper(
    //         &self,
    //     ) -> Result<Arc<Mutex<dyn Wrapper>>, Error> {
            // MockWrapper::
            // Ok(Arc::new(Mutex::new(MockWrapper::new()))
        // }

        // async get_manifest(&self, options: Option<GetManifestOptions>) -> Result<WrapManifest, Error> {

        // }
    // }
}

#[test]
fn test_packages() {

}

