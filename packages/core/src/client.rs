use crate::invoke::Invoker;
use crate::loader::Loader;
use crate::plugins::PluginRegistration;
use crate::uri::Uri;
use crate::uri_resolver::{UriResolver, UriResolverHandler};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UriRedirect {
  pub from: Uri,
  pub to: Uri,
}

impl UriRedirect {
  pub fn new(from: Uri, to: Uri) -> Self {
    Self { from, to }
  }
}

pub struct ClientConfig {
  pub redirects: Vec<UriRedirect>,
  pub resolver: Arc<dyn UriResolver>,
  pub plugins: Vec<PluginRegistration>,
}

#[async_trait(?Send)]
pub trait Client: Send + Sync + Invoker + UriResolverHandler + Loader {
  fn get_config(&self) -> &ClientConfig;
  fn get_redirects(&self) -> &Vec<UriRedirect>;
  fn get_uri_resolver(&self) -> &dyn UriResolver;
  fn get_plugins(&self) -> &Vec<PluginRegistration>;
  fn get_plugin_by_uri(&self, uri: &Uri) -> Option<&PluginRegistration>;
}
