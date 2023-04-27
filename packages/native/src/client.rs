use std::{sync::Arc, collections::HashMap};

use polywrap_client::{
    client::PolywrapClient,
    core::{invoke::Invoker, loader::Loader, uri::Uri},
};

use crate::{loader::FFILoader, invoker::FFIInvoker, wrapper::FFIWrapper};

pub struct FFIClient {
    inner_invoker: FFIInvoker,
    inner_loader: FFILoader
}

impl FFIClient {
    pub fn new(client: PolywrapClient) -> FFIClient {
        let client = Arc::new(client);

        Self {
          inner_invoker: FFIInvoker { inner_invoker: client.clone() as Arc<dyn Invoker> },
          inner_loader: FFILoader::new(client as Arc<dyn Loader>)
        }
    }

    pub fn invoke_raw(
        &self,
        uri: Arc<Uri>,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<String>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        self.inner_invoker.invoke_raw(uri, method, args, env)
    }

    pub fn get_implementations(&self, uri: Arc<Uri>) -> Result<Vec<Arc<Uri>>, polywrap_client::core::error::Error> {
        self.inner_invoker.get_implementations(uri)
    }

    pub fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<Uri>>>> {
      self.inner_invoker.get_interfaces()
    }

    pub fn get_env_by_uri(
        &self,
        uri: Arc<Uri>,
    ) -> Option<String> {
        self.inner_loader.get_env_by_uri(uri)
    }

    pub fn load_wrapper(
      &self,
      uri: Arc<Uri>
    ) -> Result<Arc<FFIWrapper>, polywrap_client::core::error::Error> {
      let loader = self.inner_loader.load_wrapper(uri)?;

      Ok(loader)
    }
}
