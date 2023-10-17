use std::sync::{Arc, Mutex};

use crate::{
    error::Error,
    resolution::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri::Uri,
};

/// Trait that defines an object that can handle URI resolution.
pub trait UriResolverHandler {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<UriPackageOrWrapper, Error>;
}
