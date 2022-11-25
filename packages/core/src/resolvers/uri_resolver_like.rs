use crate::client::UriRedirect;

use super::{uri_resolver::UriResolver, uri_resolution_context::{UriWrapper, UriPackage}};

pub enum UriResolverLike {
  Resolver(Box<dyn UriResolver>),
  Redirect(UriRedirect),
  Package(UriPackage),
  Wrapper(UriWrapper),
  ResolverLike(Vec<UriResolverLike>),
}