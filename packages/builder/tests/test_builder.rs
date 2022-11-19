use std::collections::HashMap;

use polywrap_client_builder::config_builder::BuilderConfig;
use polywrap_core::uri::Uri;
use serde_json::json;

#[test]
fn test_env_methods() {
    let mut builder = BuilderConfig::new(None);
    let foo_value = json!({
        "foo": "bar"
    });
    let uri = Uri::new("wrap://ens/simple-wrapper.eth");

    assert_eq!(builder.envs.is_none(), true);
    builder.add_env(uri.clone(), foo_value.clone());
    assert_eq!(builder.envs.is_some(), true);

    let mut simple_env = HashMap::new();
    simple_env.insert(uri.clone().uri, foo_value);

    let current_env = builder.envs.unwrap();
    let env_from_builder = current_env.into_iter().find(|env| env == &simple_env);

    assert_eq!(env_from_builder.is_some(), true);
    assert_eq!(env_from_builder.unwrap(), simple_env);

    let foo_value = json!({
        "foo": "bar"
    });
    let bar_value = json!({
        "bar": "foo"
    });

    let mut builder = BuilderConfig::new(None);

    builder.add_env(uri.clone(), foo_value.clone());
    builder.add_env(uri.clone(), bar_value.clone());

    
    let mut mixed_env = HashMap::new();
    mixed_env.insert(uri.clone().uri, json!({
        "foo": "bar",
        "bar": "foo"
    }));
    let current_env = builder.envs.unwrap();
    let env_from_builder = current_env.into_iter().find(|env| env == &mixed_env);
    assert_eq!(env_from_builder.unwrap(), mixed_env);

    let mut builder = BuilderConfig::new(None);

    builder.add_env(uri.clone(), foo_value.clone());
    builder.add_env(uri.clone(), bar_value.clone());

    
    let mut mixed_env = HashMap::new();
    mixed_env.insert(uri.clone().uri, json!({
        "foo": "bar",
        "bar": "foo"
    }));
    builder.remove_env(uri.clone());
    let current_env = builder.envs.unwrap();

    assert_eq!(current_env.len(), 0);
}
