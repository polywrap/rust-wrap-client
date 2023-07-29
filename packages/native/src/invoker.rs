use polywrap_client::core::invoker::Invoker;
use std::{collections::HashMap, sync::Arc};

use crate::{error::FFIError, resolvers::resolution_context::FFIUriResolutionContext, uri::FFIUri};

pub struct FFIInvoker(pub Arc<dyn Invoker>);

impl FFIInvoker {
    pub fn invoke_raw(
        &self,
        uri: Arc<FFIUri>,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        resolution_context: Option<Arc<FFIUriResolutionContext>>,
    ) -> Result<Vec<u8>, FFIError> {
        Ok(self.0.invoke_raw(
            &uri.0,
            &method,
            args.as_deref(),
            env.as_deref(),
            resolution_context.map(|ctx| ctx.0.clone()),
        )?)
    }

    pub fn get_implementations(&self, uri: Arc<FFIUri>) -> Result<Vec<Arc<FFIUri>>, FFIError> {
        let uris = self.0.get_implementations(&uri.0)?;
        let uris: Vec<Arc<FFIUri>> = uris
            .into_iter()
            .map(|uri| Arc::new(FFIUri(uri.clone())))
            .collect();

        Ok(uris)
    }

    pub fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<FFIUri>>>> {
        let interfaces = self.0.get_interfaces();
        let interfaces = interfaces.map(|interfaces| {
            interfaces
                .into_iter()
                .map(|(key, value)| {
                    (
                        key.to_string(),
                        value
                            .into_iter()
                            .map(|uri| Arc::new(FFIUri(uri.clone())))
                            .collect(),
                    )
                })
                .collect()
        });

        interfaces
    }

    pub fn get_env_by_uri(&self, uri: Arc<FFIUri>) -> Option<Vec<u8>> {
        self.0.get_env_by_uri(&uri.0).map(|env| env.to_vec())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use polywrap_client::core::{macros::uri, uri::Uri};
    use polywrap_tests_utils::mocks::get_mock_invoker;

    use crate::{invoker::FFIInvoker, uri::ffi_uri_from_string};

    #[test]
    fn test_ffi_invoker() {
        let ffi_invoker = FFIInvoker(get_mock_invoker());

        let uri = ffi_uri_from_string("mock/a").unwrap();
        let response = ffi_invoker.invoke_raw(uri, "foo".to_string(), None, None, None);
        assert_eq!(response.unwrap(), vec![3]);
    }

    #[test]
    fn test_ffi_get_implementations() {
        let ffi_invoker = FFIInvoker(get_mock_invoker());

        let uri = ffi_uri_from_string("mock/a").unwrap();
        let response = ffi_invoker.get_implementations(uri.clone());
        assert_eq!(response.unwrap(), vec![uri]);
    }

    #[test]
    fn test_ffi_get_interfaces() {
        let ffi_invoker = FFIInvoker(get_mock_invoker());

        let response = ffi_invoker.get_interfaces();
        assert_eq!(
            response.unwrap(),
            HashMap::from([(
                "wrap://mock/a".to_string(),
                vec![ffi_uri_from_string("mock/b").unwrap()]
            )])
        );
    }

    #[test]
    fn test_get_env_by_uri() {
        let ffi_invoker = FFIInvoker(get_mock_invoker());
        let ffi_uri = ffi_uri_from_string("mock/a");
        let response = ffi_invoker.get_env_by_uri(ffi_uri.unwrap());
        assert_eq!(response.unwrap(), [0, 4]);
    }
}
