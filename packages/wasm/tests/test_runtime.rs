use std::path::Path;
use polywrap_wasm::{wasm_wrapper::{WasmWrapper}, wasm_runtime::instance::WasmModule};
use polywrap_core::{
    wrapper::Wrapper,
    invoke::{Invoker,InvokeOptions,InvokeArgs},
    uri::Uri,
    error::Error,
    file_reader::{SimpleFileReader}
};
use polywrap_manifest::{
    deserialize::deserialize_wrap_manifest
};
use async_trait::async_trait;
use polywrap_msgpack::msgpack;
use std::sync::Arc;
use std::fs;
use polywrap_tests::helpers::get_tests_path;

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
    async fn invoke_wrapper(
        &self,
        options: &InvokeOptions,
        mut wrapper: Box<dyn Wrapper>
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper.invoke(options, Arc::new(self.clone())).await;

        if result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                result.err().unwrap()
            )));
        };

        let result = result.unwrap();

        Ok(result)    
    }
 
    async fn invoke(&self, options: &InvokeOptions) -> Result<Vec<u8>, Error> {
        let invoke_opts = InvokeOptions {
            uri: options.uri,
            args: options.args,
            method: options.method,
            resolution_context: options.resolution_context,
            env: None,
        };

        let invoke_result = self.invoke_wrapper(&invoke_opts, Box::new(self.wrapper.clone())).await;
        if invoke_result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                invoke_result.err().unwrap()
            )));
        };

        Ok(invoke_result.unwrap())
    }
}

#[tokio::test]
async fn invoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.wasm", path);
    let module = WasmModule::Path(module_path);

    let manifest_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.info", path);
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();
    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();

    let file_reader = SimpleFileReader::new();
    let wrapper = WasmWrapper::new(module, Arc::new(file_reader), manifest);
    let args = InvokeArgs::Msgpack(msgpack!({ "a": 1, "b": 1}));
    
    let invoke_opts = InvokeOptions {
        args: Some(&args),
        env: None,
        resolution_context: None,
        method: "add",
        uri: &Uri::from_string("fs/tests/cases/simple-invoke").unwrap()
    };
    
    let mock_invoker = MockInvoker::new(wrapper);
    let result = mock_invoker.invoke(&invoke_opts).await.unwrap();
    assert_eq!(result, [2])
}

