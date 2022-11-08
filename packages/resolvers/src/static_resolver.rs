use async_trait::async_trait;
use std::collections::HashMap;
use polywrap_core::{
    error::Error,
    uri_resolution_context::{ UriPackageOrWrapper },
    uri_resolver::UriResolver,
    uri_resolution_result::{ UriResolutionResult, PackageOrWrapper }
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
                    uri_map.insert(uri.clone().uri, UriPackageOrWrapper::Wrapper(w))
                }
                UriResolverLike::Package(p) => {
                    uri_map.insert(uri.clone().uri, UriPackageOrWrapper::Package(p))     
                }
                UriResolverLike::UriResolver(uri) => {
                    uri_map.insert(uri.clone().uri, UriPackageOrWrapper::Uri(uri))
                }
                UriResolverLike::UriResolverLike(resolvers) => {
                    for (uri, uri_package_or_wrapper) in resolvers.uri_map.into_iter() {
                        uri_map.insert(uri, uri_package_or_wrapper)
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
        let mut result: UriPackageOrWrapper = UriResolutionResult.ok(uri, None);
        let mut description = format!("StaticResolver - Miss");

        let uri_package_or_resolver: Some<UriPackageOrWrapper> = self.uri_map.get(uri.uri);
        if let Some(p) = uri_package_or_resolver {
            match p {
                UriPackageOrWrapper::Wrapper(wrapper) => {
                    result = UriResolutionResult.ok(uri, Some(wrapper));
                    description = format!("StaticResolver - Wrapper {}", uri.uri);
                },
                UriPackageOrWrapper::Package(package) => {
                    result = UriResolutionResult.ok(uri, Some(package));
                    description = format!("StaticResolver - Package {}", uri.uri);
                },
                UriPackageOrWrapper::Uri(uri) => {
                    description = format!("StaticResolver - Redirect {} - {}", uri.uri, p.uri.uri);
                }
            };
        }

        resolution_context.track_step(UriResolutionStep {
            source_uri: uri,
            result,
            description
        });

        return result
    }
}