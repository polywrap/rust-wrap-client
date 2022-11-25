use std::sync::Arc;

use crate::wrap::types::Http_Module_ArgsGet;
use async_trait::async_trait;
use polywrap_core::{error::Error, invoke::Invoker};
use wrap::{
    module::{ArgsGetFile, ArgsTryResolveUri, Module},
    types::{HttpModule, HttpRequest, MaybeUriOrManifest},
};
pub mod wrap;

pub struct HttpResolverPlugin {}

#[async_trait]
impl Module for HttpResolverPlugin {
    async fn try_resolve_uri(
        &mut self,
        args: &ArgsTryResolveUri,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<MaybeUriOrManifest>, Error> {
        if args.authority != "http" && args.authority != "https" {
            return Ok(None);
        };

        let manifest_search_pattern = "wrap.info";

        let get_result = HttpModule::get(
            &Http_Module_ArgsGet {
                url: format!("{}/{}", args.path, manifest_search_pattern),
                request: Some(HttpRequest {
                    response_type: wrap::types::HttpResponseType::BINARY,
                    headers: None,
                    body: None,
                    url_params: None,
                }),
            },
            invoker,
        )
        .await;

        let manifest = match get_result {
            Ok(opt_response) => {
                if let Some(response) = opt_response {
                    response.body.map(|body| base64::decode(body).unwrap())
                } else {
                    None
                }
            }
            Err(_) => {
                // TODO: logging
                // https://github.com/polywrap/monorepo/issues/33
                None
            }
        };

        Ok(Some(MaybeUriOrManifest {
            uri: None,
            manifest,
        }))
    }

    async fn get_file(
        &mut self,
        args: &ArgsGetFile,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<Vec<u8>>, Error> {
        let resolve_result = HttpModule::get(
            &Http_Module_ArgsGet {
                url: args.path.clone(),
                request: Some(HttpRequest {
                    response_type: wrap::types::HttpResponseType::BINARY,
                    headers: None,
                    body: None,
                    url_params: None,
                }),
            },
            invoker,
        )
        .await;

        let file = if let Ok(opt_result) = resolve_result {
            if let Some(result) = opt_result {
                result.body.map(|body| base64::decode(body).unwrap())
            } else {
                None
            }
        } else {
            None
        };

        Ok(file)
    }
}

impl_traits!(HttpResolverPlugin);
