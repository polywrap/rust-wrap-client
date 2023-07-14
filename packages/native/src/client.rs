use std::{collections::HashMap, ops::DerefMut, sync::Arc};

use polywrap_client::core::{client::Client, invoker::Invoker};

use crate::{
    error::FFIError,
    invoker::FFIInvoker,
    resolvers::{resolution_context::FFIUriResolutionContext, uri_package_or_wrapper::FFIUriPackageOrWrapper},
    uri::FFIUri,
    wrapper::{FFIWrapper, WrapperWrapping},
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

    pub fn as_invoker(&self) -> Arc<FFIInvoker> {
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

    pub fn try_resolve_uri(
      &self,
      uri: Arc<FFIUri>,
      resolution_context: Option<Arc<FFIUriResolutionContext>>,
    ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
      let result = self.inner_client.try_resolve_uri(
        &uri.0,
        resolution_context.map(|r| r.0.clone())
      )?;

      Ok(Arc::new(FFIUriPackageOrWrapper(result)))
    }
}

impl Invoker for FFIClient {
    fn invoke_raw(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<
            Arc<
                std::sync::Mutex<
                    polywrap_client::core::resolution::uri_resolution_context::UriResolutionContext,
                >,
            >,
        >,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        self.inner_client
            .invoke_raw(uri, method, args, env, resolution_context)
    }

    fn get_implementations(
        &self,
        uri: &polywrap_client::core::uri::Uri,
    ) -> Result<Vec<polywrap_client::core::uri::Uri>, polywrap_client::core::error::Error> {
        self.inner_client.get_implementations(uri)
    }

    fn get_interfaces(
        &self,
    ) -> Option<polywrap_client::core::interface_implementation::InterfaceImplementations> {
        self.inner_client.get_interfaces()
    }

    fn get_env_by_uri(&self, uri: &polywrap_client::core::uri::Uri) -> Option<Vec<u8>> {
        self.inner_client.get_env_by_uri(uri)
    }
}

#[cfg(test)]
mod test {
    use polywrap_client::builder::PolywrapClientConfigBuilder;
    use polywrap_client::{builder::PolywrapClientConfig, client::PolywrapClient};
    use std::{collections::HashMap, sync::Arc};

    use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};
    use polywrap_msgpack_serde::to_vec;
    use polywrap_tests_utils::mocks::{get_mock_client, get_mock_invoker, get_mock_wrapper};
    use serde::Serialize;

    use crate::uri::ffi_uri_from_string;
    use crate::{client::FFIClient, invoker::FFIInvoker, wrapper::FFIWrapper};

    #[test]
    fn ffi_invoke_raw() {
        let ffi_client = FFIClient::new(get_mock_client());
        let uri = ffi_uri_from_string("mock/a").unwrap();
        let response = ffi_client.invoke_raw(uri, "", None, None, None);
        assert_eq!(response.unwrap(), vec![5]);
    }

    #[test]
    fn ffi_load_wrapper() {
        let ffi_client = FFIClient::new(get_mock_client());
        let ffi_invoker = Arc::new(FFIInvoker(get_mock_invoker()));
        let uri = ffi_uri_from_string("mock/a").unwrap();
        let wrapper = ffi_client.load_wrapper(uri, None).unwrap();
        let response = wrapper.invoke("foo".to_string(), None, None, ffi_invoker);

        assert_eq!(response.unwrap(), vec![195]);
    }

    #[test]
    fn ffi_invoke_wrapper_raw() {
        let ffi_client = FFIClient::new(get_mock_client());
        let ffi_wrapper: Box<dyn FFIWrapper> = Box::new(get_mock_wrapper());
        let uri = ffi_uri_from_string("mock/a").unwrap();

        let response = ffi_client.invoke_wrapper_raw(ffi_wrapper, uri, "", None, None, None);
        assert_eq!(response.unwrap(), vec![6]);
    }

    #[test]
    fn ffi_get_implementations() {
        let ffi_client = FFIClient::new(get_mock_client());
        let uri = ffi_uri_from_string("mock/c").unwrap();
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
                vec![ffi_uri_from_string("mock/d").unwrap()]
            )])
        );
    }

    #[test]
    fn ffi_get_env_by_uri() {
        let ffi_client = FFIClient::new(get_mock_client());
        let uri = ffi_uri_from_string("mock/c").unwrap();

        let response = ffi_client.get_env_by_uri(uri);
        assert_eq!(response.unwrap(), [4, 8]);
    }

    #[derive(Serialize)]
    pub struct AddArgs {
        pub a: u32,
        pub b: u32,
    }

    #[test]
    fn ffi_invoke_raw_real() {
        let mut config = PolywrapClientConfig::new();
        config
            .add(SystemClientConfig::default().into())
            .add(Web3ClientConfig::default().into());

        let client = Arc::from(PolywrapClient::new(config.into()));
        let ffi_client = FFIClient::new(client.clone());

        const SUBINVOKE_WRAP_URI: &str =
            "wrap://ipfs/Qmf7jukQhTQekdSgKfdnFtB6ERTN6V7aT4oYpzesDyr2cS";
        let uri = ffi_uri_from_string(SUBINVOKE_WRAP_URI).unwrap();

        let result = ffi_client
            .invoke_raw(
                uri.clone(),
                "add",
                Some(to_vec(&AddArgs { a: 2, b: 40 }).unwrap()),
                None,
                None,
            )
            .unwrap();

        assert_eq!(result, to_vec(&42).unwrap());
    }
}
