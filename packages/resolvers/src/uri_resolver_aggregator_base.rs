use std::sync::Arc;

use polywrap_core::error::Error;

use polywrap_core::{uri::Uri, invoker::Invoker};

use polywrap_core::resolvers::{
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriResolutionStep},
    uri_resolver::UriResolver,
};

pub trait UriResolverAggregatorBase: UriResolver + core::fmt::Debug {
    fn get_resolver_name(&self) -> Option<String>;
    fn get_uri_resolvers(
        &self,
        uri: &Uri,
        client: &dyn Invoker,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<Vec<Arc<dyn UriResolver>>, Error>;
    fn get_step_description(
        &self,
        uri: &Uri,
        result: &Result<UriPackageOrWrapper, Error>,
    ) -> String;
    fn try_resolve_uri_with_resolvers(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolvers: Vec<Arc<dyn UriResolver>>,
        resolution_context: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        let sub_context = resolution_context.create_sub_history_context();
        for resolver in resolvers.into_iter() {
            let result = resolver
                .try_resolve_uri(uri, invoker.clone(), resolution_context);
            let track_and_return = if let Ok(UriPackageOrWrapper::Uri(result_uri)) = &result {
                uri.to_string() != result_uri.to_string()
            } else {
                true
            };

            if track_and_return {
                resolution_context.track_step(UriResolutionStep {
                    source_uri: uri.clone(),
                    result: result.clone(),
                    sub_history: Some(sub_context.get_history().clone()),
                    description: Some(self.get_step_description(uri, &result)),
                });

                return result;
            }
        }

        let result = Ok(UriPackageOrWrapper::Uri(uri.clone()));

        resolution_context.track_step(UriResolutionStep {
            source_uri: uri.clone(),
            result: result.clone(),
            sub_history: Some(sub_context.get_history().clone()),
            description: Some(self.get_step_description(uri, &result)),
        });

        result
    }
}
