use std::{path::Path, collections::HashMap};
use polywrap_wasm::{wasm_wrapper::{WasmWrapper}};
use polywrap_core::{
    invoke::{Invoker},
    uri::Uri,
    error::Error,
    file_reader::{SimpleFileReader}, resolvers::uri_resolution_context::UriResolutionContext, wrapper::Wrapper, env::Env, interface_implementation::InterfaceImplementations
};
use wrap_manifest_schemas::{
    deserialize::deserialize_wrap_manifest
};
use async_trait::async_trait;
use polywrap_msgpack::msgpack;
use std::sync::Arc;
use std::fs;
use polywrap_tests_utils::helpers::get_tests_path;
use futures::lock::Mutex;

#[derive(Clone)]
struct MockInvoker {
    wrapper: WasmWrapper
}

impl MockInvoker {
    fn new(wrapper: WasmWrapper) -> Self {
        Self { wrapper }
    }
}

#[async_trait]
impl Invoker for MockInvoker {
    async fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper.lock().await.invoke(
            Arc::new(self.clone()),
            uri,
            method,
            args,
            env,
            resolution_context
        ).await;

        if result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                result.err().unwrap()
            )));
        };

        let result = result.unwrap();

        Ok(result)    
    }

    async fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let invoke_result = self.invoke_wrapper_raw(
            Arc::new(Mutex::new(self.wrapper.clone())),
            uri,
            method,
            args,
            env,
            resolution_context,
        ).await;

        if invoke_result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                invoke_result.err().unwrap()
            )));
        };

        Ok(invoke_result.unwrap())
    }

    async fn get_implementations(&self, _uri: Uri) -> Result<Vec<Uri>, Error> {
        Ok(vec![])
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        let i = HashMap::new();
        Some(i)
    }
}

#[tokio::test]
async fn invoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.wasm", path);
    let manifest_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.info", path);

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();
    
    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();

    let wrapper = WasmWrapper::new(module_bytes, Arc::new(file_reader), manifest);

    let mock_invoker = MockInvoker::new(wrapper);
    let result = mock_invoker.invoke_raw(
        &Uri::try_from("ens/wrapper.eth").unwrap(),
        "add",
        Some(&msgpack!({ "a": 1, "b": 1})), 
        None,
        None
    ).await.unwrap();
    assert_eq!(result, [2])
}

