use std::{collections::HashMap, sync::Arc};

use polywrap_client::{
    client::PolywrapClient,
    core::{invoke::Invoker, loader::Loader, uri::Uri},
};
use serde_json::Value;

pub struct FFIClient {
    inner_client: PolywrapClient,
}

impl FFIClient {
    pub fn new(client: PolywrapClient) -> FFIClient {
        Self {
            inner_client: client,
        }
    }

    pub fn invoke_raw(
        &self,
        uri: &str,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<&str>,
    ) -> Vec<u8> {
        let uri: Uri = uri.to_string().try_into().unwrap();
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

        self.inner_client
            .invoke_raw(
                &uri.to_string().try_into().unwrap(),
                method,
                args,
                env,
                None,
            )
            .unwrap()
    }

    pub fn get_implementations(&self, uri: &str) -> Vec<String> {
        self.inner_client
            .get_implementations(uri.to_string().try_into().unwrap())
            .unwrap()
            .into_iter()
            .map(|u| u.to_string())
            .collect()
    }

    pub fn get_interfaces(&self) -> Option<HashMap<String, Vec<String>>> {
        if let Some(map) = self.inner_client.get_interfaces() {
            Some(
                map.into_iter()
                    .map(|(name, uris)| {
                        (
                            name,
                            uris.into_iter()
                                .map(|u| u.to_string())
                                .collect::<Vec<String>>(),
                        )
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn load_wrapper(
        &self,
        uri: &str
    ) -> Result<
        Arc<std::sync::Mutex<Box<dyn polywrap_client::core::wrapper::Wrapper>>>,
        polywrap_client::core::error::Error,
    > {
        self.inner_client.load_wrapper(&uri.try_into().unwrap(), None)
    }

    pub fn get_env_by_uri(
        &self,
        uri: &str,
    ) -> Option<String> {
        if let Some(env) = self.inner_client.get_env_by_uri(&uri.try_into().unwrap()) {
          Some(serde_json::to_string(env).unwrap())
        } else {
          None
        }
    }
}
