use std::sync::Arc;

use crate::{
    loader::Loader,
    uri::Uri,
};
use async_trait::async_trait;

use super::{uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper, UriResolutionStep}, uri_resolver::UriResolver};

#[async_trait]
pub trait UriResolverAggregatorBase: UriResolver {
    fn get_resolver_name(&self) -> Option<String>;
    async fn get_uri_resolvers(
        &self,
        uri: &Uri,
        loader: &dyn Loader,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<Vec<Arc<dyn UriResolver>>, crate::error::Error>;
    async fn get_step_descrption(
        &self,
        uri: &Uri,
        result: &Result<UriPackageOrWrapper, crate::error::Error>,
    ) -> String;
    async fn try_resolve_uri_with_resolvers(
        &self,
        uri: &Uri,
        loader: &dyn Loader,
        resolvers: Vec<Arc<dyn UriResolver>>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, crate::error::Error> {
        let sub_context = resolution_context.create_sub_history_context();

        for resolver in resolvers.into_iter() {
            let result = resolver
                .try_resolve_uri(uri, loader, resolution_context)
                .await;
            let track_and_return = if let Ok(result_value) = &result {
                if let UriPackageOrWrapper::Uri(result_uri) = result_value {
                    uri.to_string() != result_uri.to_string()
                } else {
                    true
                }
            } else {
                true
            };

            if track_and_return {
                resolution_context.track_step(UriResolutionStep {
                    source_uri: uri.clone(),
                    result: result.clone(),
                    sub_history: Some(sub_context.get_history().clone()),
                    description: Some(self.get_step_descrption(uri, &result).await),
                });

                return result;
            }
        }

        let result = Ok(UriPackageOrWrapper::Uri(uri.clone()));

        resolution_context.track_step(UriResolutionStep {
            source_uri: uri.clone(),
            result: result.clone(),
            sub_history: Some(sub_context.get_history().clone()),
            description: Some(self.get_step_descrption(uri, &result).await),
        });

        result
    }
}


