use std::sync::{Mutex, Arc};

use crate::{client::UriRedirect, wrapper::Wrapper, package::WrapPackage, uri::Uri};

use super::{uri_resolver::UriResolver};

#[derive(Clone)]
pub enum UriResolverLike {
  Resolver(Arc<dyn UriResolver>),
  Redirect(UriRedirect),
  Package(Uri, Arc<Mutex<Box<dyn WrapPackage>>>),
  Wrapper(Uri, Arc<Mutex<Box<dyn Wrapper>>>),
  ResolverLike(Vec<UriResolverLike>),
}