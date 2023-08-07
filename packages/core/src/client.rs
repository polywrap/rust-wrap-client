use std::collections::HashMap;
use std::sync::Arc;

use crate::interface_implementation::InterfaceImplementations;
use crate::invoker::Invoker;
use crate::resolution::uri_resolver::UriResolver;
use crate::uri::Uri;
use crate::uri_resolver_handler::UriResolverHandler;
use crate::wrap_invoker::WrapInvoker;
use crate::wrap_loader::WrapLoader;

/// A utility struct to store a URI redirect.
#[derive(Clone, Debug, PartialEq)]
pub struct UriRedirect {
    /// Source URI
    pub from: Uri,
    /// Destination URI
    pub to: Uri,
}

impl UriRedirect {
    pub fn new(from: Uri, to: Uri) -> Self {
        Self { from, to }
    }
}

/// Allows conversion from a tuple of URIs to a UriRedirect.
impl From<(Uri, Uri)> for UriRedirect {
    fn from((from, to): (Uri, Uri)) -> Self {
        Self { from, to }
    }
}

/// Allows conversion from a tuple of URI references to a UriRedirect.
impl From<(&Uri, &Uri)> for UriRedirect {
    fn from((from, to): (&Uri, &Uri)) -> Self {
        UriRedirect::new(from.to_owned(), to.to_owned())
    }
}

/// Configuration struct for implementors of `Client`.
/// Can be built manually or through the `ClientConfigBuilder`
#[derive(Debug)]
pub struct ClientConfig {
    pub resolver: Arc<dyn UriResolver>,
    /// Environment variables configuration.
    /// Should be a `HashMap` of `Uri` keys and msgpack buffer values
    pub envs: Option<HashMap<Uri, Vec<u8>>>,
    /// Interface implementations
    pub interfaces: Option<InterfaceImplementations>,
}

/// Defines a type that can build a `ClientConfig`.
pub trait ClientConfigBuilder {
    /// Builds a `ClientConfig` instance.
    fn build(self) -> ClientConfig;
}

pub trait Client: Invoker + WrapLoader + WrapInvoker + UriResolverHandler {}