use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{uri::Uri, loader::Loader, package::WrapPackage};

use super::{resolver_with_history::ResolverWithHistory, uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext}};

pub struct PackageResolver {
  pub uri: Uri,
  pub package: Arc<Mutex<dyn WrapPackage>>
}

impl PackageResolver {}

#[async_trait]
impl ResolverWithHistory for PackageResolver {
  fn get_step_description(&self, _: &crate::uri::Uri) -> String {
      format!("Package ({})", self.uri)
  }

  async fn _try_resolve_uri(&self, uri: &Uri, _: &dyn Loader, _: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, crate::error::Error> {
    if uri.to_string() != self.uri.to_string() {
      Ok(UriPackageOrWrapper::Uri(uri.clone()))
    } else {
      Ok(UriPackageOrWrapper::Package(uri.clone(), self.package.clone()))
    }
  }
}