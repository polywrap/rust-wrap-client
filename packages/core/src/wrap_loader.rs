use std::sync::{Arc, Mutex};

use crate::{resolution::uri_resolution_context::UriResolutionContext, error::Error, wrapper::Wrapper, uri::Uri};

pub trait WrapLoader: Send + Sync {
  fn load_wrapper(
      &self,
      uri: &Uri,
      resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
  ) -> Result<Arc<dyn Wrapper>, Error>;
}
