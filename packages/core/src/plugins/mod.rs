use std::sync::Arc;

use crate::uri::Uri;

use self::plugin_module::PluginModule;

pub mod plugin_wrapper;
pub mod plugin_module;
pub mod plugin_package;

pub struct PluginRegistration {
  pub uri: Uri,
  pub plugin: Arc<dyn PluginModule>,
}