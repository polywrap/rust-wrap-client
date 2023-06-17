use std::collections::HashMap;
use std::sync::Arc;

use crate::interface_implementation::InterfaceImplementations;
use crate::invoker::Invoker;
use crate::resolution::uri_resolver::UriResolver;
use crate::uri::Uri;
use crate::uri_resolver_handler::UriResolverHandler;
use crate::wrap_invoker::WrapInvoker;
use crate::wrap_loader::WrapLoader;

#[derive(Clone, Debug, PartialEq)]
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
    pub envs: Option<HashMap<String, Vec<u8>>>,
    pub interfaces: Option<InterfaceImplementations>,
}

pub trait Client: Invoker + WrapLoader + WrapInvoker + UriResolverHandler {}
