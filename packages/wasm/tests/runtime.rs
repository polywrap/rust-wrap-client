use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    file_reader::SimpleFileReader,
    invoke::{InvokeArgs, Invoker},
    uri::Uri,
    uri_resolution_context::UriResolutionContext,
    wrapper::Wrapper,
};
use polywrap_manifest::deserialize::deserialize_wrap_manifest;
use polywrap_msgpack::msgpack;
use polywrap_tests::helpers::get_tests_path;
use polywrap_wasm::{wasm_runtime::instance::WasmModule, wasm_wrapper::WasmWrapper};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct MockInvoker {
    wrapper: WasmWrapper,
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
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper
            .lock()
            .await
            .invoke(
                Arc::new(self.clone()),
                uri,
                method,
                args,
                resolution_context,
            )
            .await;

        if result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                result.err().unwrap()
            )));
        };

        let result = result.unwrap();

        Ok(result)
    }

    async fn invoke(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let invoke_result = self
            .invoke_wrapper(
                Arc::new(Mutex::new(self.wrapper.clone())),
                uri,
                method,
                args,
                resolution_context,
            )
            .await;
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

    let module_path = format!(
        "{}/subinvoke/00-subinvoke/implementations/as/wrap.wasm",
        path
    );
    let module = WasmModule::Path(module_path);

    let manifest_path = format!(
        "{}/subinvoke/00-subinvoke/implementations/as/wrap.info",
        path
    );
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();
    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();

    let file_reader = SimpleFileReader::new();
    let wrapper = WasmWrapper::new(module, Arc::new(file_reader), manifest);
    let args = InvokeArgs::Msgpack(msgpack!({ "a": 1, "b": 1}));

    let mock_invoker = MockInvoker::new(wrapper);
    let result = mock_invoker
        .invoke(
            &Uri::from_string("fs/tests/cases/simple-invoke").unwrap(),
            "add",
            Some(&args),
            None,
        )
        .await
        .unwrap();
    assert_eq!(result, [2])
}
