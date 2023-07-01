use std::{fmt::Debug, sync::Arc};

use polywrap_client::core::{
    error::Error,
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};

use crate::{error::FFIError, invoker::FFIInvoker};

pub trait IFFIWrapper: Debug + Send + Sync {
    fn ffi_invoke(
        &self,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError>;
}

impl IFFIWrapper for Arc<dyn Wrapper> {
    fn ffi_invoke(
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
        self.0.ffi_invoke(method.to_owned(), args, env, invoker)
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

        Ok(self
            .0
            .ffi_invoke(method.to_string(), args, env, Arc::new(FFIInvoker(invoker)))?)
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
        unimplemented!("FFI Wrapper does not implement get_file")
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use polywrap_client::core::wrapper::Wrapper;
    use polywrap_msgpack_serde::from_slice;
    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_wrapper};

    use crate::{invoker::FFIInvoker, wrapper::FFIWrapper};

    use super::IFFIWrapper;

    fn get_mocks() -> (Box<dyn IFFIWrapper>, FFIInvoker) {
        (Box::new(get_mock_wrapper()), FFIInvoker(get_mock_invoker()))
    }

    #[test]
    fn ffi_wrapper() {
        let (ffi_wrapper, ffi_invoker) = get_mocks();
        let response = ffi_wrapper.ffi_invoke("foo".to_string(), None, None, Arc::new(ffi_invoker));
        assert!(from_slice::<bool>(&response.unwrap()).unwrap());
    }

    #[test]
    fn test_ext_wrapper() {
        let (ffi_wrapper, _) = get_mocks();
        let ext_wrapper = FFIWrapper(ffi_wrapper);
        let response = 
            Wrapper::invoke(&ext_wrapper, "foo", None, None, get_mock_invoker())
            .unwrap();
        assert!(from_slice::<bool>(&response).unwrap());
    }
}
