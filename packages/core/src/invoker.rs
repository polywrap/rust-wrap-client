use crate::{
    error::Error, interface_implementation::InterfaceImplementations,
    resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
};
pub trait Invoker: Send + Sync {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        context: Option<InvokerContext>
    ) -> Result<Vec<u8>, Error>;
    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error>;
    fn get_interfaces(&self) -> Option<InterfaceImplementations>;
    fn get_env_by_uri(&self, uri: &Uri) -> Option<Vec<u8>>;
}

pub struct InvokerContext<'a> {
    pub resolution_context: &'a mut UriResolutionContext,
    pub env: Option<&'a [u8]>,
    pub caller_context: Option<&'a UriResolutionContext>,
    pub overriden_own_context: Option<&'a UriResolutionContext>
}

impl<'a> InvokerContext<'a> {
    pub fn default(resolution_context: &'a mut UriResolutionContext) -> Self {
        InvokerContextBuilder::default(resolution_context).build()
    }
}

pub struct InvokerContextBuilder<'a> (InvokerContext<'a>);

impl<'a> InvokerContextBuilder<'a> {
    pub fn default(resolution_context: &'a mut UriResolutionContext) -> Self {
        Self(
            InvokerContext {
                resolution_context,
                env: None,
                caller_context: None,
                overriden_own_context: None
            }
        )
    }

    pub fn build(self) -> InvokerContext<'a> {
        self.0
    }

    pub fn env(mut self, env: Option<&'a [u8]>) -> InvokerContextBuilder {
        self.0.env = env;
        self
    }

    pub fn caller_context(mut self, caller_context: Option<&'a UriResolutionContext>) -> InvokerContextBuilder {
        self.0.caller_context = caller_context;
        self
    }

    pub fn overriden_own_context(mut self, overriden_own_context: Option<&'a UriResolutionContext>) -> InvokerContextBuilder {
        self.0.overriden_own_context = overriden_own_context;
        self
    }
}
