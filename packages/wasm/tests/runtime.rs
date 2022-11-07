use polywrap_wasm::{wasm_wrapper::{WasmWrapper, WasmWrapperConfig}, wasm_runtime::instance::WasmModule};
use polywrap_core::{
    wrapper::Wrapper,
    invoke::{Invoker,InvokeOptions,InvokeArgs},
    uri::Uri,
    error::Error
};
use async_trait::async_trait;
use polywrap_msgpack::msgpack;
use std::sync::Arc;

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
        wrapper: Box<dyn Wrapper>
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
    let module = WasmModule::Path("./tests/cases/simple-invoke/wrap.wasm".to_string());
    let config = WasmWrapperConfig {
        wasm_module: module
    };
    let wrapper = WasmWrapper::new(config);
    let mock_invoker = MockInvoker::new(wrapper);
    let args = InvokeArgs::Msgpack(msgpack!({ "a": 1, "b": 1}));

    let invoke_opts = InvokeOptions {
        args: Some(&args),
        env: None,
        resolution_context: None,
        method: "add",
        uri: &Uri::from_string("fs/tests/cases/simple-invoke").unwrap()
    };

    let result = mock_invoker.invoke(&invoke_opts).await.unwrap();
    assert_eq!(result, [2])
}

