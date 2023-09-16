use std::sync::{Arc, Mutex};

use polywrap_core::{
    error::Error, interface_implementation::InterfaceImplementations, invoker::Invoker,
    resolution::{uri_resolution_context::UriResolutionContext, get_uri_resolution_path::get_uri_resolution_path}, uri::Uri,
};
use polywrap_msgpack_serde::{to_vec, from_slice};
use polywrap_plugin::{InvokerContext, InvokerContextBuilder};
use serde::{Deserialize, Serialize};

pub struct Subinvoker {
    resolution_context: UriResolutionContext,
    caller_resolution_context: Option<UriResolutionContext>,
    overriden_own_context: Option<UriResolutionContext>,
    subinvocation_context: Arc<Mutex<UriResolutionContext>>,
    invoker: Arc<dyn Invoker>,
}

impl Subinvoker {
    pub fn new(
        invoker: Arc<dyn Invoker>,
        resolution_context: UriResolutionContext,
        caller_resolution_context: Option<UriResolutionContext>,
        overriden_own_context: Option<UriResolutionContext>,
        subinvocation_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Self {
        Self {
            invoker,
            resolution_context,
            caller_resolution_context,
            overriden_own_context,
            subinvocation_context
        }
    }
}

impl Invoker for Subinvoker {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        _context: Option<InvokerContext>
    ) -> Result<Vec<u8>, Error> {
        let resolution_context = self.resolution_context.clone();
        let mut subinvocation_context = self.subinvocation_context.lock().unwrap().clone();

        match uri.to_string().as_str() {
            "wrap://https/http.wrappers.dev/u/test/invocation-context" => invocation_context(method, &resolution_context, &self.caller_resolution_context, &self.overriden_own_context),
            "wrap://https/http.wrappers.dev/u/test/delegate-subinvoke" => delegate_subinvoke(self.invoker.clone(), method, args, &mut subinvocation_context, &self.resolution_context, self.caller_resolution_context.clone()),
            _ => self.invoker.invoke_raw(
                uri, 
                method, 
                args, 
                Some(InvokerContextBuilder::default(&mut subinvocation_context)
                    .caller_context(Some(&resolution_context))
                    .build())
            )
        }
    }
    fn get_implementations(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
        self.invoker.get_implementations(uri)
    }
    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        self.invoker.get_interfaces()
    }
    fn get_env_by_uri(&self, uri: &Uri) -> Option<Vec<u8>> {
        self.invoker.get_env_by_uri(uri)
    }
}

pub fn invocation_context(method: &str, resolution_context: &UriResolutionContext, caller_resolution_context: &Option<UriResolutionContext>, overriden_own_context: &Option<UriResolutionContext>) -> Result<Vec<u8>, Error> {
    match method {
        "getOwnContext" => {
            let invocation_context = to_vec(
                &get_invocation_context(&Some(overriden_own_context.as_ref().unwrap_or(resolution_context)))?
            ).unwrap();

            Ok(invocation_context)
        },
        "getCallerContext" => {
            let invocation_context = to_vec(
                &get_invocation_context(&caller_resolution_context.as_ref())?
            ).unwrap();

            Ok(invocation_context)
        }
        _ => {
            return Err(Error::WrapperError(format!("Method {} not found", method)));
        }
    }
}

fn get_invocation_context(resolution_context: &Option<&UriResolutionContext>) -> Result<Option<InvocationContext>, Error> {
    if !resolution_context.is_some() {
        return Ok(None);
    }

    let history = resolution_context.as_ref().unwrap().get_history().get(0).unwrap().sub_history.as_ref().unwrap();
    let resolution_path: Vec<String> = get_uri_resolution_path(&history).iter().map(|x| x.source_uri.to_string())
        .collect();

    let origin_uri = if resolution_path.len() > 0 {
        resolution_path.first().unwrap().to_string()
    } else {
        "".to_string()
    };
    let final_uri = if resolution_path.len() > 0 {
        resolution_path.last().unwrap().to_string()
    } else {
        "".to_string()
    };
    Ok(Some(InvocationContext {
        origin_uri,
        final_uri,
    }))
}

pub fn delegate_subinvoke(invoker: Arc<dyn Invoker>, method: &str, args: Option<&[u8]>, subinvocation_context: &mut UriResolutionContext, resolution_context: &UriResolutionContext, caller_resolution_context: Option<UriResolutionContext>) -> Result<Vec<u8>, Error> {
    match method {
        "subinvoke" => {
            let args: DelegateSubinvokeArgs = match args {
                Some(args) => from_slice(args).map_err(|e| Error::WrapperError(format!("Error invoking delegate-subinvoke: {}", e.to_string())))?,
                None => return Err(Error::WrapperError(format!("Empty DelegateSubinvokeArgs")))
            };

            invoker.invoke_raw(&args.uri.parse().unwrap(), &args.method, Some(&args.args),
                Some(
                    InvokerContextBuilder::default(subinvocation_context)
                        .caller_context(caller_resolution_context.as_ref())
                        .overriden_own_context(Some(resolution_context))
                        .build()
                )
            )
        },
        _ => {
            return Err(Error::WrapperError(format!("Method {} not found", method)));
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DelegateSubinvokeArgs {
    pub uri: String,
    pub method: String,
    pub args: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct InvocationContext {
    #[serde(rename = "originUri")]
    pub origin_uri: String,
    #[serde(rename = "finalUri")]
    pub final_uri: String,
}

struct InvocationContexts {
    caller_context: Option<InvocationContext>,
    overriden_own_context: Option<InvocationContext>
}