use std::{fmt::Debug, sync::Arc};

use polywrap_client::core::{
    error::Error,
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};

use crate::{error::FFIError, invoker::FFIInvoker};

pub trait FFIWrapper: Debug + Send + Sync {
    fn invoke(
        &self,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError>;
}

impl FFIWrapper for Arc<dyn Wrapper> {
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
pub struct WrapperWrapping(pub Box<dyn FFIWrapper>);

impl Wrapper for WrapperWrapping {
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, Error> {
        let args = args.map(|args| args.to_vec());
        let env = env.map(|env| env.to_vec());

        Ok(self.0.invoke(
            method.to_string(),
            args,
            env,
            Arc::new(FFIInvoker(invoker)),
        )?)
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
        unimplemented!("FFI Wrapper does not implement get_file")
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use polywrap_client::{core::wrapper::Wrapper};
    use polywrap_msgpack_serde::from_slice;
    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_wrapper};

    use crate::{invoker::FFIInvoker, wrapper::WrapperWrapping};

    use super::FFIWrapper;

    fn get_mocks() -> (Box<dyn FFIWrapper>, FFIInvoker) {
        (Box::new(get_mock_wrapper()), FFIInvoker(get_mock_invoker()))
    }

    #[test]
    fn ffi_wrapper() {
        let (ffi_wrapper, ffi_invoker) = get_mocks();
        let response =
            ffi_wrapper.invoke("foo".to_string(), None, None, Arc::new(ffi_invoker));
        assert!(from_slice::<bool>(&response.unwrap()).unwrap());
    }

    #[test]
    fn test_ext_wrapper() {
        let (ffi_wrapper, _) = get_mocks();
        let ext_wrapper = WrapperWrapping(ffi_wrapper);
        let response = ext_wrapper
            .invoke("foo", None, None, get_mock_invoker())
            .unwrap();
        assert!(from_slice::<bool>(&response).unwrap());
    }
}
