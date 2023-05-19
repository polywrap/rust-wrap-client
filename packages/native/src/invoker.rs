use polywrap_client::core::{invoker::Invoker, uri::Uri};
use std::{collections::HashMap, sync::Arc};

use crate::uri::FFIUri;

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
        uri: Arc<FFIUri>,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        let args = args.as_deref();
        let env = env.as_deref();

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
        uri: Arc<FFIUri>,
    ) -> Result<Vec<Arc<FFIUri>>, polywrap_client::core::error::Error> {
        Ok(self
            .inner_invoker
            .get_implementations(&uri.0)?
            .into_iter()
            .map(|uri| Arc::new(uri.into()))
            .collect())
    }

    pub fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<FFIUri>>>> {
        if let Some(interfaces) = self.inner_invoker.get_interfaces() {
            let interfaces = interfaces
                .into_iter()
                .map(|(key, uris)| {
                    let uris = uris.into_iter().map(|uri| Arc::new(uri.into())).collect();
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
      env: Option<&[u8]>,
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

  fn get_env_by_uri(&self, uri: &Uri) -> Option<&[u8]> {
      self.inner_invoker.get_env_by_uri(uri)
  }
}
