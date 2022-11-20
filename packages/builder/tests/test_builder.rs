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

    let mut mixed_env = HashMap::new();
    mixed_env.insert(uri.clone().uri, json!({
        "foo": "bar",
        "bar": "foo"
    }));
    let mut builder = BuilderConfig::new(None);
    builder.add_envs(mixed_env.clone());
    let current_env = builder.envs.unwrap();
    let env_from_builder = current_env.into_iter().find(|env| env == &mixed_env);
    assert_eq!(env_from_builder.unwrap(), mixed_env);

    let mut builder = BuilderConfig::new(None);
    builder.add_env(uri.clone(), json!({ "foo": "bar" }));
    builder.remove_env(uri.clone());
    assert_eq!(builder.envs.unwrap().len(), 0);

    let mut builder = BuilderConfig::new(None);
    builder.add_env(uri.clone(), json!({ "foo": "bar" }));
    builder.set_env(uri.clone(), json!({ "bar": "foo" }));
    let mut expected_env = HashMap::new();
    expected_env.insert(uri.clone().uri, json!({ "bar": "foo" }));
    assert_eq!(builder.envs.unwrap().first().unwrap(), &expected_env);
}
