use std::collections::HashMap;

use polywrap_client::{core::resolvers::{
    recursive_resolver::RecursiveResolver, static_resolver::StaticResolver,
    uri_resolution_context::UriPackageOrWrapper as InnerUriPackageOrWrapper,
    uri_resolver_like::UriResolverLike as InnerUriResolverLike,
}, resolvers::extendable_uri_resolver::ExtendableUriResolver};

use self::{
    uri_package_or_wrapper::UriPackageOrWrapper,
    uri_resolver_like::UriResolverLike,
};

pub mod uri_package_or_wrapper;
pub mod uri_resolver_like;

pub fn create_extendable_uri_resolver(name: Option<String>) -> ExtendableUriResolver {
  ExtendableUriResolver::new(name)
}

pub fn create_recursive_resolver(uri_resolver_like: UriResolverLike) -> RecursiveResolver {
    let uri_resolver_like: InnerUriResolverLike = uri_resolver_like.into();
    RecursiveResolver::from(uri_resolver_like)
}

pub fn create_static_resolver(
    uri_map: HashMap<String, UriPackageOrWrapper>,
) -> StaticResolver {
    let uri_map: HashMap<String, InnerUriPackageOrWrapper> = uri_map
        .into_iter()
        .map(|(uri, variant)| (uri, variant.into()))
        .collect();

    StaticResolver::new(uri_map)
}
