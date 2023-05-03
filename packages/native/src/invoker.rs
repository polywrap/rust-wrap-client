use polywrap_client::core::{invoke::Invoker, uri::Uri};
use serde_json::Value;
use std::{sync::Arc, collections::HashMap};

pub struct FFIInvoker {
    pub inner_invoker: Arc<dyn Invoker>,
}

impl FFIInvoker {
    pub fn invoke_raw(
        &self,
        uri: Arc<Uri>,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<String>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        let args = args.as_deref();

        let env = env.map(|env| serde_json::from_str::<Value>(&env).unwrap());

        self.inner_invoker
            .invoke_raw(
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
        Ok(
          self.inner_invoker
          .get_implementations(uri.as_ref().clone())?
          .into_iter()
          .map(|uri| uri.into())
          .collect()
        )
    }

    pub fn get_interfaces(
        &self,
    ) -> Option<HashMap<String, Vec<Arc<Uri>>>> {
        if let Some(interfaces) = self.inner_invoker.get_interfaces() {
          let interfaces = interfaces.into_iter().map(|(key, uris)| {
            let uris = uris.into_iter().map(|uri| uri.into()).collect();
            (key, uris)
          }).collect();

          Some(interfaces)
        } else {
          None
        }
    }
}

impl From<Arc<dyn Invoker>> for FFIInvoker {
    fn from(value: Arc<dyn Invoker>) -> Self {
        FFIInvoker {
            inner_invoker: value,
        }
    }
}
