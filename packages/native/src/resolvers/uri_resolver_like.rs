use std::sync::{Arc, Mutex};

use polywrap_client::core::{
    client::UriRedirect,
    package::WrapPackage,
    resolvers::{
        uri_resolver::UriResolver,
        uri_resolver_like::UriResolverLike as InnerUriResolverLike,
    },
    wrapper::Wrapper,
};

pub enum UriResolverLike {
    Resolver { resolver: Box<dyn UriResolver> },
    Redirect { from: String, to: String },
    Package { uri: String, package: Box<dyn WrapPackage> },
    Wrapper { uri: String, wrapper: Box<dyn Wrapper> },
    ResolverLike { resolvers: Vec<UriResolverLike> },
}

impl From<UriResolverLike> for InnerUriResolverLike {
  fn from(value: UriResolverLike) -> Self {
      match value {
          UriResolverLike::Resolver { resolver } => {
              InnerUriResolverLike::Resolver(Arc::from(resolver))
          }
          UriResolverLike::Redirect { from, to } => {
              InnerUriResolverLike::Redirect(UriRedirect { 
                from: from.try_into().unwrap(),
                to: to.try_into().unwrap()
              })
          }
          UriResolverLike::Package { uri, package } => InnerUriResolverLike::Package(
              uri.try_into().unwrap(),
              Arc::new(Mutex::new(package)),
          ),
          UriResolverLike::Wrapper { uri, wrapper } => InnerUriResolverLike::Wrapper(
              uri.try_into().unwrap(),
              Arc::new(Mutex::new(wrapper)),
          ),
          UriResolverLike::ResolverLike { resolvers } => {
              InnerUriResolverLike::ResolverLike(
                  resolvers
                      .into_iter()
                      .map(|resolver_like| resolver_like.into())
                      .collect(),
              )
          }
      }
  }
}

