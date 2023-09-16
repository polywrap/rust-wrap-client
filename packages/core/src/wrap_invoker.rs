use crate::{
    error::Error, uri::Uri,
    wrapper::Wrapper, invoker::InvokerContext,
};

pub trait WrapInvoker: Send + Sync {
    fn invoke_wrapper_raw(
        &self,
        wrapper: &dyn Wrapper,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        context: Option<InvokerContext>,
    ) -> Result<Vec<u8>, Error>;
}
