use std::sync::{Arc, Mutex};

use crate::{
    error::Error,
    resolution::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    uri::Uri,
};

pub trait UriResolverHandler {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<UriPackageOrWrapper, Error>;
}
