use polywrap_client::{core::invoke::Invoker};
use serde_json::Value;
use std::sync::Arc;

pub struct FFIInvoker {
  pub inner_client: Arc<dyn Invoker>
}

impl FFIInvoker {
  pub fn invoke_raw(&self, uri: &str, method: &str, args: Option<Vec<u8>>, env: Option<String>) -> Vec<u8> {
    let args = if let Some(args) = &args {
      Some(args.as_slice())
    } else {
      None
    };
    
    let env = if let Some(env) = env {
      Some(serde_json::from_str::<Value>(&env).unwrap())
    } else {
      None
    };

    self.inner_client.invoke_raw(
      &uri.to_string().try_into().unwrap(),
      method,
      args,
      env,
      None
    ).unwrap()
  }
}

impl Invoker for FFIInvoker {
    fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<std::sync::Mutex<Box<dyn polywrap_client::core::wrapper::Wrapper>>>,
        uri: &polywrap_client::core::uri::Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<polywrap_client::core::env::Env>,
        resolution_context: Option<&mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        self.inner_client.invoke_wrapper_raw(wrapper, uri, method, args, env, resolution_context)
    }

    fn invoke_raw(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<polywrap_client::core::env::Env>,
        resolution_context: Option<&mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        self.inner_client.invoke_raw(uri, method, args, env, resolution_context)
    }

    fn get_implementations(&self, uri: polywrap_client::core::uri::Uri) -> Result<Vec<polywrap_client::core::uri::Uri>, polywrap_client::core::error::Error> {
        self.inner_client.get_implementations(uri)
    }

    fn get_interfaces(&self) -> Option<polywrap_client::core::interface_implementation::InterfaceImplementations> {
        self.inner_client.get_interfaces()
    }
}

impl From<Arc<dyn Invoker>> for FFIInvoker {
    fn from(value: Arc<dyn Invoker>) -> Self {
        FFIInvoker { inner_client: value }
    }
}
