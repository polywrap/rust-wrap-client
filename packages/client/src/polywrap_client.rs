use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use futures::executor::block_on;
use polywrap_core::{
    client::{Client, ClientConfig, UriRedirect},
    error::CoreError,
    invoke::{InvokeOptions, InvokerOptions, Invoker},
    uri::{
        uri::Uri,
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::{UriResolver, UriResolverHandler},
    },
    wrapper::{GetFileOptions, Wrapper},
};

use crate::error::ClientError;

pub struct PolywrapClient {
    config: Arc<ClientConfig>,
    callback:
        Option<Arc<Mutex<dyn FnMut(InvokerOptions) -> Result<Vec<u8>, CoreError> + Send + Sync>>>,
}

impl PolywrapClient {
    pub fn new(config: Arc<ClientConfig>) -> Self {
      let config_clone = config.clone();
      let mock_client = Self {
          config: config_clone,
          callback: None,
      };

      let invoke = Arc::new(Mutex::new(move |options: InvokerOptions| {
          block_on(mock_client.invoke(&options))
      }));

      Self {
          config,
          callback: Some(invoke),
      }
    }

    pub async fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&UriResolutionContext>,
    ) -> Result<Box<dyn Wrapper>, ClientError> {
        let empty_res_context = UriResolutionContext::new();
        let resolution_ctx = match resolution_context {
            Some(ctx) => ctx,
            None => &empty_res_context,
        };

        let result = self.try_resolve_uri(uri, Some(resolution_ctx)).await;

        // TODO: Handle errors
        if result.is_err() {
            return Err(ClientError::LoadWrapperError(format!(
                "Failed to resolve wrapper: {}",
                result.err().unwrap()
            )));
        };

        let uri_package_or_wrapper = result.unwrap();

        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri) => {
                return Err(ClientError::LoadWrapperError(format!(
                    "Failed to resolve wrapper: {}",
                    uri
                )));
            }
            UriPackageOrWrapper::Wrapper(_, wrapper) => {
                return Ok(wrapper.wrapper);
            }
            UriPackageOrWrapper::Package(_, package) => {
                let wrapper = package.package.create_wrapper().await.unwrap();
                return Ok(wrapper);
            }
        }
    }
}

#[async_trait(?Send)]
impl Invoker for PolywrapClient {
    async fn invoke(&self, options: &InvokerOptions) -> Result<Vec<u8>, CoreError> {
        let empty_res_context = UriResolutionContext::new();
        let resolution_context = match &options.invoke_options.resolution_context {
            None => &empty_res_context,
            Some(ctx) => ctx,
        };

        let uri = options.invoke_options.uri;

        let load_wrapper_result = self.load_wrapper(&uri, Some(resolution_context)).await;

        if load_wrapper_result.is_err() {
            return Err(CoreError::InvokeError(format!(
                "Failed to load wrapper: {}",
                load_wrapper_result.err().unwrap()
            )));
        };

        let wrapper = load_wrapper_result.unwrap();
        let invoke_opts = InvokeOptions {
            uri: &uri,
            args: options.invoke_options.args,
            method: options.invoke_options.method,
            resolution_context: options.invoke_options.resolution_context,
            env: None,
        };

        let opts = InvokerOptions {
            invoke_options: invoke_opts,
            encode_result: options.encode_result,
        };

        let invoke_result = self.invoke_wrapper(&opts, wrapper).await;

        if invoke_result.is_err() {
            return Err(CoreError::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                invoke_result.err().unwrap()
            )));
        };

        Ok(invoke_result.unwrap())
    }

    async fn invoke_wrapper(
        &self,
        options: &InvokerOptions,
        mut wrapper: Box<dyn Wrapper>,
    ) -> Result<Vec<u8>, CoreError> {
        let sd = self.callback.as_ref().unwrap();
        let result = wrapper.invoke(&options.invoke_options, sd.clone());

        if result.is_err() {
            return Err(CoreError::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                result.err().unwrap()
            )));
        };

        let result = result.unwrap();

        Ok(result)
    }
}

#[async_trait(?Send)]
impl Client for PolywrapClient {
    fn get_config(&self) -> &ClientConfig {
        &self.config
    }

    fn get_redirects(&self) -> &Vec<UriRedirect> {
        &self.config.redirects
    }

    fn get_uri_resolver(&self) -> &Box<dyn UriResolver> {
        &self.config.resolver
    }

    async fn get_file(&self, uri: &Uri, options: &GetFileOptions) -> Result<Vec<u8>, CoreError> {
        let load = self.load_wrapper(&uri, Option::None).await;

        match load {
            Ok(wrapper) => {
                let result = wrapper.get_file(&options);
                return result;
            }
            Err(err) => {
                return Err(CoreError::GetFileError(format!(
                    "Failed to load wrapper: {}",
                    err
                )));
            }
        }
    }
}

#[async_trait(?Send)]
impl UriResolverHandler for PolywrapClient {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&UriResolutionContext>,
    ) -> Result<polywrap_core::uri::uri_resolution_context::UriPackageOrWrapper, CoreError> {
        let uri_resolver = self.get_uri_resolver();
        let uri_resolver_context = UriResolutionContext::new();

        let resolution_context = match resolution_context {
            Some(ctx) => ctx,
            None => &uri_resolver_context,
        };

        let res = uri_resolver
            .try_resolve_uri(uri, self, resolution_context)
            .await;

        res
    }
}
