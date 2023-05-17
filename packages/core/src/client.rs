use std::sync::Arc;

use crate::uri::Uri;
use crate::interface_implementation::InterfaceImplementations;
use crate::resolvers::uri_resolver::UriResolver;
use crate::env::Envs;
use crate::invoker::Invoker;
use crate::uri_resolver_handler::UriResolverHandler;
use crate::wrap_invoker::WrapInvoker;
use crate::wrap_loader::WrapLoader;

#[derive(Clone,Debug)]
pub struct UriRedirect {
  pub from: Uri,
  pub to: Uri,
}

impl UriRedirect {
  pub fn new(from: Uri, to: Uri) -> Self {
    Self { from, to }
  }
}

#[derive(Debug)]
pub struct ClientConfig {
  pub resolver: Arc<dyn UriResolver>,
  pub envs: Option<Envs>,
  pub interfaces: Option<InterfaceImplementations>
}

pub trait Client: Invoker + WrapLoader + WrapInvoker + UriResolverHandler {}
