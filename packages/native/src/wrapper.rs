use std::{fmt::Debug, sync::Arc};

use polywrap_client::core::{
    error::Error,
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};

use crate::{error::FFIError, invoker::FFIInvoker};

pub trait IFFIWrapper: Debug + Send + Sync {
    fn invoke(
        &self,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError>;
}

impl IFFIWrapper for Arc<dyn Wrapper> {
    fn invoke(
        &self,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError> {
        let arc_self = self.clone();

        Ok(Wrapper::invoke(
            arc_self.as_ref(),
            &method,
            args.as_deref(),
            env.as_deref(),
            invoker.0.clone(),
        )?)
    }
}

#[derive(Debug)]
pub struct FFIWrapper(pub Box<dyn IFFIWrapper>);

impl FFIWrapper {
    pub fn new(wrapper: Box<dyn IFFIWrapper>) -> Self {
        Self(wrapper)
    }

    pub fn invoke(
        &self,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError> {
        self.0.invoke(method.to_string(), args, env, invoker)
    }
}

impl Wrapper for FFIWrapper {
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, Error> {
        let args = args.map(|args| args.to_vec());
        let env = env.map(|env| env.to_vec());

        self.0
            .invoke(method.to_string(), args, env, Arc::new(FFIInvoker(invoker)))
            .map_err(|e| e.into())
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
        unimplemented!("FFI Wrapper does not implement get_file")
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_client::core::{error::Error, wrapper::Wrapper};
    use polywrap_msgpack_serde::from_slice;
    use polywrap_tests_utils::mocks::get_mock_invoker;

    use crate::{
        error::FFIError, invoker::FFIInvoker, mocks::wrapper::get_mock_ffi_wrapper,
        wrapper::FFIWrapper,
    };

    fn get_mocks() -> (FFIWrapper, FFIInvoker) {
        (get_mock_ffi_wrapper(), FFIInvoker(get_mock_invoker()))
    }

    #[test]
    fn ffi_wrapper() {
        let (ffi_wrapper, ffi_invoker) = get_mocks();
        let response: Result<Vec<u8>, crate::error::FFIError> =
            ffi_wrapper.invoke("foo", None, None, Arc::new(ffi_invoker));
        assert!(from_slice::<bool>(&response.unwrap()).unwrap());
    }

    #[test]
    fn ffi_wrapper_invocation_with_error() {
        let (ffi_wrapper, ffi_invoker) = get_mocks();
        let response = ffi_wrapper.invoke("error_method", None, None, Arc::new(ffi_invoker));
        assert!(response.is_err());
        let error = response.unwrap_err();
        match error {
            FFIError::InvokeError { uri, method, err } => {
                assert_eq!(uri, "mock/ffi-wrap");
                assert_eq!(method, "error_method");
                assert_eq!(err, "error from mock ffi wrapper");
            }
            _ => panic!("Unexpected error type received"),
        }
    }

    #[test]
    fn wrapper_invoke_passing_ffi_wrapper() {
        let (ffi_wrapper, _) = get_mocks();
        let response = Wrapper::invoke(&ffi_wrapper, "foo", None, None, get_mock_invoker());
        assert!(from_slice::<bool>(&response.unwrap()).unwrap());
    }

    #[test]
    fn wrapper_invocake_with_error_passing_ffi_wrapper() {
        let (ffi_wrapper, _) = get_mocks();
        let response = Wrapper::invoke(&ffi_wrapper, "error_method", None, None, get_mock_invoker());
        assert!(response.is_err());
        let error = response.unwrap_err();
        match error {
            Error::InvokeError(uri, method, err) => {
                assert_eq!(uri, "mock/ffi-wrap");
                assert_eq!(method, "error_method");
                assert_eq!(err, "error from mock ffi wrapper");
            }
            _ => panic!("Unexpected error type received"),
        }
    }
}
