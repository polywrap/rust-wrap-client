use async_trait::async_trait;
use std::collections::HashMap;
use polywrap_core::{
    error::Error,
    uri_resolution_context::{ 
        UriPackageOrWrapper, UriWrapper, UriPackage, UriResolutionContext, UriResolutionStep
    },
    uri_resolver::UriResolver,
    uri::Uri,
    loader::Loader
};
use crate::helpers::UriResolverLike;
use crate::helpers::UriResolverLike::Package;

type ResolverMap = HashMap<String, UriPackageOrWrapper>;

struct StaticResolver {
    uri_map: ResolverMap
}

impl StaticResolver {
    fn from(static_resolver_likes: Vec<UriResolverLike>) -> Self {
        let mut uri_map: ResolverMap = HashMap::new();
        for static_resolver in static_resolver_likes.iter() {
            match static_resolver {
                UriResolverLike::Wrapper(w) => {
                    uri_map.insert(static_resolver.uri, UriPackageOrWrapper::Wrapper(
                        UriWrapper {   
                            uri: static_resolver.uri,
                            wrapper: Box::new(w)
                        }
                    ))
                }
                UriResolverLike::Package(p) => {
                    uri_map.insert(static_resolver.uri, UriPackageOrWrapper::Package(
                        UriPackage {
                            uri: static_resolver.uri,
                            package: p
                        }
                    ))     
                }
                UriResolverLike::UriResolver(uri) => {
                    uri_map.insert(static_resolver.uri, UriPackageOrWrapper::Uri(uri))
                }
                UriResolverLike::UriResolverLike(resolvers) => {
                    let resolver = StaticResolver::from(resolvers.to_vec());
                    for (uri, uri_package_or_wrapper) in resolver.uri_map.into_iter() {
                        uri_map.insert(uri, uri_package_or_wrapper);
                    }
                }
            };
        }
        return StaticResolver::new(uri_map)
    }

    fn new(uri_map: ResolverMap) -> Self {
        Self {
            uri_map
        }
    }
}

#[async_trait]
impl UriResolver for StaticResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: &dyn Loader,
        resolution_context: &UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let mut description = format!("StaticResolver - Miss");

        let uri_package_or_resolver = self.uri_map.get(&uri.uri).clone();
        if let Some(p) = uri_package_or_resolver {
            match p {
                UriPackageOrWrapper::Wrapper(wrapper) => {
                    description = format!("StaticResolver - Wrapper {}", uri.uri);
                },
                UriPackageOrWrapper::Package(package) => {
                    description = format!("StaticResolver - Package {}", uri.uri);
                },
                UriPackageOrWrapper::Uri(_uri) => {
                    description = format!("StaticResolver - Redirect {} - {}", _uri.uri, uri.uri);
                }
            };
            resolution_context.track_step(UriResolutionStep {
                source_uri: uri.clone(),
                result: Ok(p),
                description: Some(description),
                sub_history: None
            });
            return Ok(p);
        }

        return Err(Error::ResolutionResultError(description));
    }
}