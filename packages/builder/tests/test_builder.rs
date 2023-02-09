use std::{collections::HashMap};

use polywrap_client_builder::types::{BuilderConfig, ClientBuilder};
use polywrap_core::{
    uri::Uri,
    client::UriRedirect
};
use serde_json::json;

#[test]
fn test_env_methods() {
    let mut builder = BuilderConfig::new(None);
    let uri = Uri::new("wrap://ens/wrapper.eth");

    assert_eq!(builder.envs.is_none(), true);

    builder.add_env(uri.clone(), json!({ "d": "d" }));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.uri);

    assert_eq!(env_from_builder.is_some(), true);
    assert_eq!(env_from_builder.unwrap(), &json!({ "d": "d" }));

    let mut envs = HashMap::new();
    envs.insert(uri.clone().uri, json!({"a": "a", "b": "b"}));

    builder.add_envs(envs);

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.uri);
    assert_eq!(env_from_builder.unwrap(), &json!({ "d": "d", "a": "a", "b": "b" }));

    builder.set_env(uri.clone(), json!({"c": "c"}));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.uri);
    assert_eq!(env_from_builder.unwrap(), &json!({ "c": "c" }));

    builder.remove_env(uri);

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
    let implementations = interfaces.get(&interface_uri.uri).unwrap();
    assert_eq!(builder.interfaces.is_some(), true);
    assert_eq!(implementations, &vec![implementation_a_uri.clone(), implementation_b_uri.clone()]);

    let implementation_c_uri = Uri::new("wrap://ens/implementation-c.eth");
    builder.add_interface_implementation(interface_uri.clone(), implementation_c_uri.clone());

    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.uri).unwrap();
    assert_eq!(implementations, &vec![
        implementation_a_uri.clone(), 
        implementation_b_uri.clone(),
        implementation_c_uri.clone()
    ]);

    builder.remove_interface_implementation(interface_uri.clone(), implementation_b_uri);
    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.uri).unwrap();
    assert_eq!(implementations, &vec![
        implementation_a_uri,
        implementation_c_uri
    ]);

}

#[test]
fn test_redirects() {
    let mut builder = BuilderConfig::new(None);
    assert_eq!(builder.redirects.is_some(), false);

    let redirects = vec![
        UriRedirect{
            from: "ens/c.eth".to_string().try_into().unwrap(), 
            to: "ens/d.eth".to_string().try_into().unwrap()
        },
        UriRedirect{
            from: "ens/f.eth".to_string().try_into().unwrap(), 
            to: "ens/g.eth".to_string().try_into().unwrap()
        },
    ];
    builder.add_redirects(redirects);

    assert_eq!(builder.redirects.is_some(), true);
    let builder_redirects = builder.redirects.unwrap();
    assert_eq!(builder_redirects[0].from, "ens/c.eth".to_string().try_into().unwrap());
    assert_eq!(builder_redirects[0].to, "ens/d.eth".to_string().try_into().unwrap());
    assert_eq!(builder_redirects[1].from, "ens/f.eth".to_string().try_into().unwrap());
    assert_eq!(builder_redirects[1].to, "ens/g.eth".to_string().try_into().unwrap());

    let mut builder = BuilderConfig::new(None);
    assert_eq!(builder.redirects.is_some(), false);

    builder.add_redirect("ens/a.eth".to_string().try_into().unwrap(), "ens/b.eth".to_string().try_into().unwrap());
    assert_eq!(builder.redirects.is_some(), true);

    builder.remove_redirect("ens/a.eth".to_string().try_into().unwrap());
    assert_eq!(builder.redirects.is_some(), false);
}

#[test]
fn test_packages() {
    let builder = BuilderConfig::new(None);
    assert_eq!(builder.packages.is_some(), false);

    
}


#[test]
fn test_wrappers() {
}