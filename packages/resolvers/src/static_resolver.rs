use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use polywrap_core::{
    error::Error,
    uri_resolution_context::{ 
        UriPackageOrWrapper, UriResolutionContext, UriResolutionStep
    },
    uri_resolver::UriResolver,
    uri::Uri,
    loader::Loader
};
use crate::{
    helpers::UriResolverLike
};

type ResolverMap = HashMap<String, Arc<UriPackageOrWrapper>>;

pub struct StaticResolver {
    uri_map: ResolverMap
}

impl StaticResolver {
    pub fn _from(static_resolver_likes: Vec<UriResolverLike>) -> StaticResolver {
        let mut uri_map: ResolverMap = HashMap::new();
        for static_resolver in static_resolver_likes.into_iter() {
            match static_resolver {
                UriResolverLike::Wrapper(w) => {
                    let uri = w.uri.uri.clone();
                    let wrapper = Arc::new(UriPackageOrWrapper::Wrapper(w));
                    uri_map.insert(uri, wrapper);
                }
                UriResolverLike::Package(p) => {
                    let uri = p.uri.uri.clone();
                    let package = Arc::new(UriPackageOrWrapper::Package(p));
                    uri_map.insert(uri, package);
                }
                UriResolverLike::UriResolver(_uri) => {
                    let uri = _uri.uri.clone();
                    let boxed_uri = Arc::new(UriPackageOrWrapper::Uri(_uri));
                    uri_map.insert(uri, boxed_uri);
                }
                UriResolverLike::UriResolverLike(uri_resolver_likes) => {
                    for uri_resolver_like in uri_resolver_likes.into_iter() {
                        let resolver = StaticResolver::_from(vec![uri_resolver_like]);
                        for (uri, uri_package_or_wrapper) in resolver.uri_map.into_iter() {
                            uri_map.insert(uri, uri_package_or_wrapper);
                        }
                    }
                }
            };
        }
        return StaticResolver::new(uri_map)
    }

    fn new(uri_map: ResolverMap) -> StaticResolver {
        StaticResolver {
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
        resolution_context: &mut UriResolutionContext,
    ) -> Result<Arc<UriPackageOrWrapper>, Error> {
        let mut description = format!("StaticResolver - Miss");

        if let Some(uri_package_or_wrapper) = self.uri_map.get(&uri.uri) {
            let u = uri_package_or_wrapper.clone();
            match u.as_ref() {
                UriPackageOrWrapper::Wrapper(_) => {
                    description = format!("StaticResolver - Wrapper {}", uri.uri);
                },
                UriPackageOrWrapper::Package(_) => {
                    description = format!("StaticResolver - Package {}", uri.uri);
                },
                UriPackageOrWrapper::Uri(_uri) => {
                    description = format!("StaticResolver - Redirect {} - {}", _uri.uri, uri.uri);
                }
            };
            resolution_context.track_step(UriResolutionStep {
                source_uri: uri.clone(),
                result: Ok(u.clone()),
                description: Some(description),
                sub_history: None
            });
            return Ok(u);
        }

        return Err(Error::ResolutionResultError(description));
    }
}