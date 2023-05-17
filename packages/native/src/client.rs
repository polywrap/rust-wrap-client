use std::{collections::HashMap, sync::Arc};

use polywrap_client::{
    core::{client::Client, error::Error},
};
use serde_json::Value;

use crate::{wrapper::FFIWrapper, uri::FFIUri};

pub struct FFIClient {
    inner_client: Arc<dyn Client>,
}

impl FFIClient {
    pub fn new(client: Arc<dyn Client>) -> FFIClient {
        Self {
            inner_client: client
        }
    }

    pub fn invoke_raw(
        &self,
        uri: Arc<FFIUri>,
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

        self.inner_client.invoke_raw(
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
            .inner_client
            .get_implementations(&uri.0)?
            .into_iter()
            .map(|uri| Arc::new(uri.into()))
            .collect())
    }

    pub fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<FFIUri>>>> {
        if let Some(interfaces) = self.inner_client.get_interfaces() {
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

    pub fn get_env_by_uri(&self, uri: Arc<FFIUri>) -> Option<String> {
        match self.inner_client.get_env_by_uri(&uri.0) {
            Some(env) => Some(env.to_string()),
            None => None,
        }
    }

    pub fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<FFIWrapper>,
        uri: Arc<FFIUri>,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<String>,
    ) -> Result<Vec<u8>, Error> {
        let args = args.as_deref();

        let mut _decoded_env = serde_json::Value::Null;
        let env = env.map(|env| {
            _decoded_env = serde_json::from_str::<Value>(&env).unwrap();
            &_decoded_env
        });

        self.inner_client.invoke_wrapper_raw(wrapper.0.clone(), &uri.0, method, args, env, None)
    }

    pub fn load_wrapper(&self, uri: Arc<FFIUri>) -> Result<Arc<FFIWrapper>, Error> {
        let wrapper = self.inner_client.load_wrapper(&uri.0, None)?;
        Ok(Arc::new(FFIWrapper::new(wrapper)))
    }
}
