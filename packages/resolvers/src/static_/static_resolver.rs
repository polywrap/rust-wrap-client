use std::collections::HashMap;

use async_trait::async_trait;
use polywrap_core::{
    client::UriRedirect,
    error::Error,
    loader::Loader,
    uri::Uri,
    uri_resolution_context::{
        UriPackage, UriPackageOrWrapper, UriResolutionContext, UriResolutionStep, UriWrapper,
    },
    uri_resolver::UriResolver,
};

pub enum StaticResolverLike {
    Redirect(UriRedirect),
    Wrapper(UriWrapper),
    Package(UriPackage),
    StaticResolverLike(Vec<StaticResolverLike>),
}

pub enum UriResolverLike {
    Resolver(Box<dyn UriResolver>),
    Redirect(UriRedirect),
    Package(UriPackage),
    Wrapper(UriWrapper),
    ResolverLike(Vec<UriResolverLike>),
}

pub struct StaticResolver {
    pub uri_map: HashMap<String, UriPackageOrWrapper>,
}

impl StaticResolver {
    pub fn new(uri_map: HashMap<String, UriPackageOrWrapper>) -> Self {
        Self { uri_map }
    }

    pub fn from(static_resolver_likes: Vec<StaticResolverLike>) -> Self {
        let mut uri_map: HashMap<String, UriPackageOrWrapper> = HashMap::new();

        for static_resolver_like in static_resolver_likes {
            match static_resolver_like {
                StaticResolverLike::StaticResolverLike(resolver_like_vec) => {
                    let resolver = StaticResolver::from(resolver_like_vec);
                    uri_map.extend(resolver.uri_map);
                }
                StaticResolverLike::Redirect(redirect) => {
                    uri_map.insert(
                        redirect.from.to_string(),
                        UriPackageOrWrapper::Uri(redirect.to),
                    );
                }
                StaticResolverLike::Package(package) => {
                    uri_map.insert(
                        package.uri.to_string(),
                        UriPackageOrWrapper::Package(package.uri.clone(), package.package),
                    );
                }
                StaticResolverLike::Wrapper(wrapper) => {
                    uri_map.insert(
                        wrapper.uri.to_string(),
                        UriPackageOrWrapper::Wrapper(wrapper.uri.clone(), wrapper.wrapper),
                    );
                }
            }
        }

        Self { uri_map }
    }
}

#[async_trait]
impl UriResolver for StaticResolver {
    async fn try_resolve_uri(
        &mut self,
        uri: &Uri,
        _: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_package_or_wrapper = self.uri_map.remove(&uri.to_string());

        let (description, result, result_uri) = if let Some(found) = uri_package_or_wrapper {
            match found {
                UriPackageOrWrapper::Package(uri, package) => (
                    format!("StaticResolver - Package ({})", uri.to_string()),
                    UriPackageOrWrapper::Package(uri.clone(), package),
                    uri,
                ),
                UriPackageOrWrapper::Wrapper(uri, wrapper) => (
                    format!("StaticResolver - Wrapper ({})", uri.to_string()),
                    UriPackageOrWrapper::Wrapper(uri.clone(), wrapper),
                    uri,
                ),
                UriPackageOrWrapper::Uri(uri) => (
                    format!("StaticResolver - Redirect ({})", uri.to_string()),
                    UriPackageOrWrapper::Uri(uri.clone()),
                    uri,
                ),
            }
        } else {
            (
                "StaticResolver - Miss".to_string(),
                UriPackageOrWrapper::Uri(uri.clone()),
                uri.clone(),
            )
        };

        resolution_context.track_step(UriResolutionStep {
            description: Some(description),
            source_uri: uri.clone(),
            result: result_uri,
            sub_history: None,
        });

        Ok(result)
    }
}
