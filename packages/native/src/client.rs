use std::{collections::HashMap, ops::DerefMut, sync::Arc};

use polywrap_client::core::{client::Client, invoker::Invoker};

use crate::{
    error::FFIError,
    resolvers::resolution_context::FFIUriResolutionContext,
    uri::FFIUri,
    wrapper::{FFIWrapper, WrapperWrapping}, invoker::FFIInvoker,
};

#[derive(Clone)]
pub struct FFIClient {
    inner_client: Arc<dyn Client>,
}

impl FFIClient {
    pub fn new(client: Arc<dyn Client>) -> FFIClient {
        Self {
            inner_client: client,
        }
    }

    pub fn as_invoker(&self) -> Arc<FFIInvoker>{
      let invoker = Arc::new(self.clone()) as Arc<dyn Invoker>;
      Arc::new(FFIInvoker(invoker))
    }

    pub fn invoke_raw(
        &self,
        uri: Arc<FFIUri>,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        resolution_context: Option<Arc<FFIUriResolutionContext>>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        let args = args.as_deref();
        let env = env.as_deref();

        self.inner_client.invoke_raw(
            &uri.to_string().try_into().unwrap(),
            method,
            args,
            env,
            resolution_context.map(|ctx| ctx.0.clone()),
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

    pub fn get_env_by_uri(&self, uri: Arc<FFIUri>) -> Option<Vec<u8>> {
        self.inner_client.get_env_by_uri(&uri.0).map(|e| e.to_vec())
    }

    pub fn invoke_wrapper_raw(
        &self,
        wrapper: Box<dyn FFIWrapper>,
        uri: Arc<FFIUri>,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        resolution_context: Option<Arc<FFIUriResolutionContext>>,
    ) -> Result<Vec<u8>, FFIError> {
        let args = args.as_deref();

        if let Some(resolution_context) = resolution_context {
            let mut res_context_guard = resolution_context.0.lock().unwrap();

            Ok(self.inner_client.invoke_wrapper_raw(
                &WrapperWrapping(wrapper),
                &uri.0,
                method,
                args.as_deref(),
                env.as_deref(),
                Some(res_context_guard.deref_mut()),
            )?)
        } else {
            Ok(self.inner_client.invoke_wrapper_raw(
                &WrapperWrapping(wrapper),
                &uri.0,
                method,
                args.as_deref(),
                env.as_deref(),
                None,
            )?)
        }
    }

    pub fn load_wrapper(
        &self,
        uri: Arc<FFIUri>,
        resolution_context: Option<Arc<FFIUriResolutionContext>>,
    ) -> Result<Box<dyn FFIWrapper>, FFIError> {
        let wrapper = self
            .inner_client
            .load_wrapper(&uri.0, resolution_context.map(|ctx| ctx.0.clone()))?;

        Ok(Box::new(wrapper))
    }
}

impl Invoker for FFIClient {
  fn invoke_raw(
      &self,
      uri: &polywrap_client::core::uri::Uri,
      method: &str,
      args: Option<&[u8]>,
      env: Option<&[u8]>,
      resolution_context: Option<Arc<std::sync::Mutex<polywrap_client::core::resolution::uri_resolution_context::UriResolutionContext>>>,
  ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
      self.inner_client.invoke_raw(uri, method, args, env, resolution_context)
  }

  fn get_implementations(&self, uri: &polywrap_client::core::uri::Uri) -> Result<Vec<polywrap_client::core::uri::Uri>, polywrap_client::core::error::Error> {
      self.inner_client.get_implementations(uri)
  }

  fn get_interfaces(&self) -> Option<polywrap_client::core::interface_implementation::InterfaceImplementations> {
      self.inner_client.get_interfaces()
  }

  fn get_env_by_uri(&self, uri: &polywrap_client::core::uri::Uri) -> Option<Vec<u8>> {
      self.inner_client.get_env_by_uri(uri)
  }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use polywrap_tests_utils::mocks::{get_mock_client, get_mock_invoker, get_mock_wrapper};

    use crate::{client::FFIClient, uri::FFIUri, wrapper::FFIWrapper, invoker::FFIInvoker};

    #[test]
    fn ffi_invoke_raw() {
        let ffi_client = FFIClient::new(get_mock_client());
        let uri = Arc::new(FFIUri::from_string("mock/a"));
        let response = ffi_client.invoke_raw(uri, "", None, None, None);
        assert_eq!(response.unwrap(), vec![5]);
    }

    #[test]
    fn ffi_load_wrapper() {
        let ffi_client = FFIClient::new(get_mock_client());
        let ffi_invoker = Arc::new(FFIInvoker(get_mock_invoker()));
        let uri = Arc::new(FFIUri::from_string("mock/a"));
        let wrapper = ffi_client.load_wrapper(uri, None).unwrap();
        let response = wrapper.invoke("foo".to_string(), None, None, ffi_invoker, None);

        assert_eq!(response.unwrap(), vec![195]);
    }

    #[test]
    fn ffi_invoke_wrapper_raw() {
        let ffi_client = FFIClient::new(get_mock_client());
        let ffi_wrapper: Box<dyn FFIWrapper> = Box::new(get_mock_wrapper());
        let uri = Arc::new(FFIUri::from_string("mock/a"));

        let response = ffi_client.invoke_wrapper_raw(ffi_wrapper, uri, "", None, None, None);
        assert_eq!(response.unwrap(), vec![6]);
    }

    #[test]
    fn ffi_get_implementations() {
        let ffi_client = FFIClient::new(get_mock_client());
        let uri = Arc::new(FFIUri::from_string("mock/c"));
        let response = ffi_client.get_implementations(uri.clone());
        assert_eq!(response.unwrap(), vec![uri]);
    }

    #[test]
    fn ffi_get_interfaces() {
        let ffi_client = FFIClient::new(get_mock_client());
        let response = ffi_client.get_interfaces();
        assert_eq!(
            response.unwrap(),
            HashMap::from([(
                ("mock/c".to_string()),
                vec![Arc::new(FFIUri::from_string("mock/d"))]
            )])
        );
    }

    #[test]
    fn ffi_get_env_by_uri() {
        let ffi_client = FFIClient::new(get_mock_client());
        let uri = Arc::new(FFIUri::from_string("mock/c"));

        let response = ffi_client.get_env_by_uri(uri);
        assert_eq!(response.unwrap(), [4, 8]);
    }
}
