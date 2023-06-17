use std::sync::{Arc, Mutex};

use crate::{
    error::Error, resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
    wrapper::Wrapper,
};

pub trait WrapLoader: Send + Sync {
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Arc<dyn Wrapper>, Error>;
}
