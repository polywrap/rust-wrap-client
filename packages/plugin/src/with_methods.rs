use std::{collections::HashMap, sync::Arc, fmt::{Debug, Formatter}};
use polywrap_core::{invoker::Invoker};

use crate::{method::PluginMethod, module::{PluginModule}, error::PluginError};

#[derive(Clone)]
pub struct PluginModuleWithMethods {
  methods_map: HashMap<String, Arc<PluginMethod>>
}

impl Default for PluginModuleWithMethods {
  fn default() -> Self {
    Self::new()
  }
}

impl PluginModuleWithMethods {
  pub fn new() -> Self {
    Self {
      methods_map: HashMap::new()
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
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, PluginError> {
        if let Some(method) = self.methods_map.get(method_name) {
          (method)(params, env, invoker.as_ref())
        } else {
          Err(PluginError::MethodNotFoundError(method_name.to_string()))
        }
    }
}

impl Debug for PluginModuleWithMethods {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, r#"
      Plugin With Methods
      
      -Methods: {:?}
      "#, self.methods_map.keys())
  }
}