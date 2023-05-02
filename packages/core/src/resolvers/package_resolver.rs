use core::fmt;
use std::sync::{Arc, Mutex};

use crate::{uri::Uri, package::WrapPackage, client::Client};

use super::{resolver_with_history::ResolverWithHistory, uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext}};

pub struct PackageResolver {
  pub uri: Uri,
  pub package: Arc<Mutex<Box<dyn WrapPackage>>>
}

impl PackageResolver {}

impl ResolverWithHistory for PackageResolver {
  fn get_step_description(&self, _: &crate::uri::Uri) -> String {
      format!("Package ({})", self.uri)
  }

  fn _try_resolve_uri(&self, uri: &Uri, _: Arc<dyn Client>, _: &mut UriResolutionContext) -> Result<UriPackageOrWrapper, crate::error::Error> {
    if uri.to_string() != self.uri.to_string() {
      Ok(UriPackageOrWrapper::Uri(uri.clone()))
    } else {
      Ok(UriPackageOrWrapper::Package(uri.clone(), self.package.clone()))
    }
  }
}

impl fmt::Debug for PackageResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "PackageResolver: {}", self.uri)
  }
}