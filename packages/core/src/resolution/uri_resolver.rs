use std::sync::Arc;
use std::{fmt::Debug, sync::Mutex};

use super::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext};
use crate::error::Error;
use crate::invoker::Invoker;
use crate::uri::Uri;

pub trait UriResolver: Send + Sync + Debug {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        client: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error>;
}
