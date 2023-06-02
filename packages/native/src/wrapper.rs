use std::{fmt::Debug, sync::Arc};

use polywrap_client::core::{
    error::Error,
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};

use crate::{
    error::FFIError,
    invoker::{FFIInvoker, FFIInvokerWrapping, InvokerWrapping},
};

pub trait FFIWrapper: Debug + Send + Sync {
    fn invoke(
        &self,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Box<dyn FFIInvoker>,
        abort_handler: Option<Box<dyn FFIAbortHandler>>,
    ) -> Result<Vec<u8>, FFIError>;
}

impl FFIWrapper for Arc<dyn Wrapper> {
    fn invoke(
        &self,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Box<dyn FFIInvoker>,
        abort_handler: Option<Box<dyn FFIAbortHandler>>,
    ) -> Result<Vec<u8>, FFIError> {
        let arc_self = self.clone();
        let abort_handler = abort_handler.map(|a| {
            Box::new(move |msg: String| a.abort(msg)) as Box<dyn Fn(String) + Send + Sync>
        });

        Ok(Wrapper::invoke(
            arc_self.as_ref(),
            &method,
            args.as_deref(),
            env.as_deref(),
            Arc::new(FFIInvokerWrapping(invoker)),
            abort_handler,
        )?)
    }
}

pub trait FFIAbortHandler: Send + Sync {
    fn abort(&self, msg: String);
}

pub struct AbortHandler(Box<dyn Fn(String) + Send + Sync>);

impl FFIAbortHandler for AbortHandler {
    fn abort(&self, msg: String) {
        self.0(msg)
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
        abort_handler: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<Vec<u8>, Error> {
        let invoker = Box::new(InvokerWrapping(invoker));
        let args = args.map(|args| args.to_vec());
        let env = env.map(|env| env.to_vec());
        let abort_handler =
            abort_handler.map(|a| Box::new(AbortHandler(a)) as Box<dyn FFIAbortHandler>);

        Ok(self
            .0
            .invoke(method.to_string(), args, env, invoker, abort_handler)?)
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
        unimplemented!("FFI Wrapper does not implement get_file")
    }
}

#[cfg(test)]
mod test {

    use polywrap_client::{core::wrapper::Wrapper, msgpack::decode};
    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_wrapper};

    use crate::{invoker::InvokerWrapping, wrapper::WrapperWrapping};

    use super::FFIWrapper;

    fn get_mocks() -> (Box<dyn FFIWrapper>, InvokerWrapping) {
        (
            Box::new(get_mock_wrapper()),
            InvokerWrapping(get_mock_invoker()),
        )
    }

    #[test]
    fn test_ffi_wrapper() {
        let (ffi_wrapper, ffi_invoker) = get_mocks();
        let response =
            ffi_wrapper.invoke("foo".to_string(), None, None, Box::new(ffi_invoker), None);
        assert!(decode::<bool>(&response.unwrap()).unwrap());
    }

    #[test]
    fn test_ext_wrapper() {
        let (ffi_wrapper, _) = get_mocks();
        let ext_wrapper = WrapperWrapping(ffi_wrapper);
        let response = ext_wrapper
            .invoke("foo", None, None, get_mock_invoker(), None)
            .unwrap();
        assert!(decode::<bool>(&response).unwrap());
    }
}
