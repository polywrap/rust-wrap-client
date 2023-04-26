use polywrap_client::core::{invoke::Invoker, uri::Uri};
use serde_json::Value;
use std::{sync::Arc};

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
    ) -> Result<Vec<Uri>, polywrap_client::core::error::Error> {
        self.inner_invoker.get_implementations(uri.as_ref().clone())
    }

    pub fn get_interfaces(
        &self,
    ) -> Option<polywrap_client::core::interface_implementation::InterfaceImplementations> {
        self.inner_invoker.get_interfaces()
    }
}

impl From<Arc<dyn Invoker>> for FFIInvoker {
    fn from(value: Arc<dyn Invoker>) -> Self {
        FFIInvoker {
            inner_invoker: value,
        }
    }
}
