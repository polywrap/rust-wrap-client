use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use polywrap_core::env::{Env};
use serde_json::Value;

use crate::{method::PluginMethod, module::{PluginModule, PluginWithEnv}, error::PluginError};

#[derive(Clone)]
pub struct PluginModuleWithMethods {
  methods_map: HashMap<String, Arc<PluginMethod>>,
  env: Env
}

impl PluginModuleWithMethods {
  pub fn new() -> Self {
    Self {
      methods_map: HashMap::new(),
      env: Value::Null
    }
  }

  pub fn methods<'a>(&'a mut self, methods: HashMap<String, Arc<PluginMethod>>) -> &'a mut Self {
    self.methods_map = methods;
    self
  }
}

#[async_trait]
impl PluginModule for PluginModuleWithMethods {
    async fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &serde_json::Value,
        invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<serde_json::Value, PluginError> {
        if let Some(method) = self.methods_map.get(method_name) {
          (method)(params.clone(), invoker)
        } else {
          Err(PluginError::MethodNotFoundError(method_name.to_string()))
        }
    }
}

#[async_trait]
impl PluginWithEnv for PluginModuleWithMethods {
    fn set_env(&mut self, env: Env) {
        self.env = env;
    }
    
    fn get_env(&self, key: String) -> Option<&Env> {
        if let Some(env) = self.env.get(&key) {
          Some(env)
        } else {
          None
        }
    }
}