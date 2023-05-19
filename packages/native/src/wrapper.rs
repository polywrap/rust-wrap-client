use std::{fmt::Debug, sync::Arc};

use polywrap_client::core::{
    error::Error,
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};

use crate::invoker::FFIInvoker;

pub trait FFIAbortHandler: Send + Sync {
    fn abort(&self, msg: String);
}

pub trait FFIWrapper: Debug + Send + Sync {
    fn invoke(
        &self,
        method: String,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
        abort_handler: Option<Box<dyn FFIAbortHandler>>,
    ) -> Vec<u8>;
}

pub struct AbortHandler(Box<dyn Fn(String) + Send + Sync>);

impl FFIAbortHandler for AbortHandler {
    fn abort(&self, msg: String) {
        self.0(msg)
    }
}

#[derive(Debug)]
pub struct ExtWrapper(pub Box<dyn FFIWrapper>);

impl Wrapper for ExtWrapper {
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
        abort_handler: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<Vec<u8>, Error> {
        let invoker = Arc::new(FFIInvoker::new(invoker));
        let args = args.map(|args| args.to_vec());
        let env = env.map(|env| env.to_vec());
        let abort_handler =
            abort_handler.map(|a| Box::new(AbortHandler(a)) as Box<dyn FFIAbortHandler>);

        Ok(self
            .0
            .invoke(method.to_string(), args, env, invoker, abort_handler))
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
        unimplemented!("FFI Wrapper does not implement get_file")
    }
}
