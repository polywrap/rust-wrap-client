use std::sync::Arc;

use crate::{
    error::Error, resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
    wrapper::Wrapper,
};

pub trait WrapLoader: Send + Sync {
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<dyn Wrapper>, Error>;
}
