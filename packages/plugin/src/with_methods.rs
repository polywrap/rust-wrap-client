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

impl Default for PluginModuleWithMethods {
  fn default() -> Self {
    Self::new()
  }
}

impl PluginModuleWithMethods {
  pub fn new() -> Self {
    Self {
      methods_map: HashMap::new(),
      env: Value::Null
    }
  }

  pub fn methods(&mut self, methods: HashMap<String, Arc<PluginMethod>>) -> &mut Self {
    self.methods_map = methods;
    self
  }
}

impl PluginModule for PluginModuleWithMethods {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        invoker: std::sync::Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<Vec<u8>, PluginError> {
        if let Some(method) = self.methods_map.get(method_name) {
          (method)(params, invoker)
        } else {
          Err(PluginError::MethodNotFoundError(method_name.to_string()))
        }
    }
}

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