use core::fmt;
use std::{collections::HashMap, sync::{Mutex, Arc}};
use crate::{
    client::UriRedirect,
    error::Error,
    loader::Loader,
    uri::Uri,
    resolvers::uri_resolution_context::{
      UriPackageOrWrapper, UriResolutionContext, UriResolutionStep,
    },
    resolvers::uri_resolver::UriResolver, package::WrapPackage, wrapper::Wrapper,
};

pub enum StaticResolverLike {
    Redirect(UriRedirect),
    Wrapper(Uri, Arc<Mutex<dyn Wrapper>>),
    Package(Uri, Arc<Mutex<dyn WrapPackage>>),
    StaticResolverLike(Vec<StaticResolverLike>),
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
                StaticResolverLike::Package(uri, package) => {
                    uri_map.insert(
                        uri.to_string(),
                        UriPackageOrWrapper::Package(uri, package),
                    );
                }
                StaticResolverLike::Wrapper(uri, wrapper) => {
                    uri_map.insert(
                        uri.to_string(),
                        UriPackageOrWrapper::Wrapper(uri.clone(), wrapper),
                    );
                }
            }
        }

        Self { uri_map }
    }
}

impl UriResolver for StaticResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_package_or_wrapper = self.uri_map.get(&uri.to_string());
        let (description, result) = if let Some(found) = uri_package_or_wrapper {
            match found {
                UriPackageOrWrapper::Package(uri, package) => (
                    format!("StaticResolver - Package ({})", uri),
                    UriPackageOrWrapper::Package(uri.clone(), package.clone())
                ),
                UriPackageOrWrapper::Wrapper(uri, wrapper) => (
                    format!("StaticResolver - Wrapper ({})", uri),
                    UriPackageOrWrapper::Wrapper(uri.clone(), wrapper.clone())
                ),
                UriPackageOrWrapper::Uri(uri) => (
                    format!("StaticResolver - Redirect ({})", uri),
                    UriPackageOrWrapper::Uri(uri.clone())
                ),
            }
        } else {
            (
                "StaticResolver - Miss".to_string(),
                UriPackageOrWrapper::Uri(uri.clone()),
            )
        };

        resolution_context.track_step(UriResolutionStep {
            description: Some(description),
            source_uri: uri.clone(),
            result: Ok(result.clone()),
            sub_history: None,
        });

        Ok(result)
    }
}

impl fmt::Debug for StaticResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "StaticResolver")
  }
}