use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use polywrap_core::{
    client::{Client, ClientConfig, UriRedirect},
    error::CoreError,
    invoke::{InvokeOptions, Invoker, InvokerOptions},
    uri::{
        uri::Uri,
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::{UriResolver, UriResolverHandler},
    },
    wrapper::{GetFileOptions, Wrapper},
};

use crate::error::ClientError;

pub struct PolywrapClient {
    config: ClientConfig,
    invoker: Arc<Mutex<Subinvoker>>,
}

#[derive(Clone)]
pub struct Subinvoker {
    loaded_wrapper: Option<Arc<dyn Wrapper>>,
}

impl Subinvoker {
    pub fn new() -> Self {
        Self {
            loaded_wrapper: None,
        }
    }

    pub fn load_wrapper(&mut self, wrapper: Arc<dyn Wrapper>) {
        self.loaded_wrapper = Some(wrapper);
    }
}

#[async_trait(?Send)]
impl Invoker for Subinvoker {
    async fn invoke_wrapper(
        &self,
        options: &InvokerOptions,
        wrapper: Arc<dyn Wrapper>,
    ) -> Result<Vec<u8>, CoreError> {
        let result = wrapper.invoke(&options.invoke_options, Arc::new(Mutex::new(self.clone())));

        if result.is_err() {
            return Err(CoreError::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                result.err().unwrap()
            )));
        };

        let result = result.unwrap();

        Ok(result)
    }

    async fn invoke(&self, options: &InvokerOptions) -> Result<Vec<u8>, CoreError> {
        let uri = options.invoke_options.uri;
        let invoke_opts = InvokeOptions {
            uri,
            args: options.invoke_options.args,
            method: options.invoke_options.method,
            resolution_context: options.invoke_options.resolution_context,
            env: None,
        };

        let opts = InvokerOptions {
            invoke_options: invoke_opts,
            encode_result: options.encode_result,
        };

        let wrapper = match self.loaded_wrapper {
            Some(ref w) => w.clone(),
            None => {
                return Err(CoreError::InvokeError(format!(
                    "No wrapper loaded for uri: {}",
                    uri
                )))
            }
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
}

impl PolywrapClient {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            invoker: Arc::new(Mutex::new(Subinvoker::new())),
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
            UriPackageOrWrapper::Uri(uri) => Err(ClientError::LoadWrapperError(format!(
                "Failed to resolve wrapper: {}",
                uri
            ))),
            UriPackageOrWrapper::Wrapper(_, wrapper) => Ok(wrapper.wrapper),
            UriPackageOrWrapper::Package(_, package) => {
                let wrapper = package.package.create_wrapper().await.unwrap();
                Ok(wrapper)
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

        let load_wrapper_result = self.load_wrapper(uri, Some(resolution_context)).await;

        if load_wrapper_result.is_err() {
            return Err(CoreError::InvokeError(format!(
                "Failed to load wrapper: {}",
                load_wrapper_result.err().unwrap()
            )));
        };

        let wrapper = load_wrapper_result.unwrap();
        let invoke_opts = InvokeOptions {
            uri,
            args: options.invoke_options.args,
            method: options.invoke_options.method,
            resolution_context: options.invoke_options.resolution_context,
            env: None,
        };

        let opts = InvokerOptions {
            invoke_options: invoke_opts,
            encode_result: options.encode_result,
        };

        let invoke_result = self.invoke_wrapper(&opts, Arc::from(wrapper)).await;

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
        wrapper: Arc<dyn Wrapper>,
    ) -> Result<Vec<u8>, CoreError> {
        let wrapper_clone = wrapper.clone();
        self.invoker.lock().unwrap().load_wrapper(wrapper_clone);

        let result = wrapper.invoke(&options.invoke_options, self.invoker.clone());

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
        let load = self.load_wrapper(uri, Option::None).await;

        match load {
            Ok(wrapper) => {
                let result = wrapper.get_file(options);
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

        uri_resolver
            .try_resolve_uri(uri, self, resolution_context)
            .await
    }
}
