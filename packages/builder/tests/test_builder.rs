use std::collections::HashMap;

use polywrap_client_builder::config_builder::BuilderConfig;
use polywrap_core::uri::Uri;
use serde_json::json;

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