use polywrap_client::core::{invoke::Invoker, uri::Uri};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

pub struct FFIInvoker {
    pub inner_invoker: Arc<dyn Invoker>,
}

impl FFIInvoker {
    pub fn new(invoker: Arc<dyn Invoker>) -> Self {
        Self {
            inner_invoker: invoker,
        }
    }

    pub fn invoke_raw(
        &self,
        uri: Arc<Uri>,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<String>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        let args = args.as_deref();

        let mut _decoded_env = serde_json::Value::Null;
        let env = env.map(|env| {
          _decoded_env = serde_json::from_str::<Value>(&env).unwrap();
          &_decoded_env
        });

        self.inner_invoker.invoke_raw(
            &uri.to_string().try_into().unwrap(),
            method,
            args,
            env,
            None,
        )
    }

    pub fn get_implementations(
        &self,
        uri: Arc<Uri>,
    ) -> Result<Vec<Arc<Uri>>, polywrap_client::core::error::Error> {
        Ok(self
            .inner_invoker
            .get_implementations(uri.as_ref())?
            .into_iter()
            .map(|uri| uri.into())
            .collect())
    }

    pub fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<Uri>>>> {
        if let Some(interfaces) = self.inner_invoker.get_interfaces() {
            let interfaces = interfaces
                .into_iter()
                .map(|(key, uris)| {
                    let uris = uris.into_iter().map(|uri| uri.into()).collect();
                    (key, uris)
                })
                .collect();

            Some(interfaces)
        } else {
            None
        }
    }
}

impl Invoker for FFIInvoker {
  fn invoke_raw(
      &self,
      uri: &Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&polywrap_client::core::env::Env>,
      resolution_context: Option<
          &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
      >,
  ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
      self.inner_invoker
          .invoke_raw(uri, method, args, env, resolution_context)
  }

  fn get_implementations(
      &self,
      uri: &Uri,
  ) -> Result<Vec<Uri>, polywrap_client::core::error::Error> {
      self.inner_invoker.get_implementations(uri)
  }

  fn get_interfaces(
      &self,
  ) -> Option<polywrap_client::core::interface_implementation::InterfaceImplementations> {
      self.inner_invoker.get_interfaces()
  }
}
