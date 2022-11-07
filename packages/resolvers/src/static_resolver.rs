use async_trait::async_trait;
use std::collections::HashMap;
use polywrap_core::{
    error::Error,
    uri_resolution_context::{ UriPackageOrWrapper },
    uri_resolver::UriResolver
};
use crate::helpers::UriResolverLike;
use crate::helpers::UriResolverLike::Package;

type ResolverMap = HashMap<String, UriPackageOrWrapper>;

struct StaticResolver {
    uri_map: ResolverMap
}

impl StaticResolver {
    fn from(static_resolver_likes: Vec<UriResolverLike>) {
        let uri_map: ResolverMap = HashMap::new();
        for static_resolver in static_resolver_likes.iter() {
            match static_resolver {
                UriResolverLike::Wrapper(w) => {}
                UriResolverLike::Package(p) => {}
                UriResolverLike::UriResolver(u) => {}
                UriResolverLike::UriResolverLike(u) => {}
            };
            Ok(())
            // uri_map.insert()
        }
    }
}

#[async_trait]
impl UriResolver for StaticResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: &dyn Loader,
        _: &UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        // let mut result
        if let Some(p) = self.uri_map.get((uri.clone()).uri) {
            Wrapper(w) => {
                
            },
            Package(p) => {

            },
            Uri(u) => {

            }
        }
    }
}