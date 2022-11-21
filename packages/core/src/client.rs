use crate::invoke::Invoker;
use crate::loader::Loader;
use crate::uri::Uri;
use crate::interface_implementation::InterfaceImplementations;
use crate::uri_resolver::{UriResolverHandler};
use crate::env::{Env};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UriRedirect {
  pub from: Uri,
  pub to: Uri,
}

impl UriRedirect {
  pub fn new(from: Uri, to: Uri) -> Self {
    Self { from, to }
  }
}

#[async_trait(?Send)]
pub trait Client: Send + Sync + Invoker + UriResolverHandler + Loader {
  fn get_redirects(&self) -> &Vec<UriRedirect>;
  fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env>;
  fn get_interfaces(&self) -> Option<&InterfaceImplementations>;
}
