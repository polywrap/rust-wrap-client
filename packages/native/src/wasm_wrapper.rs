use std::sync::Arc;

use polywrap_client::core::{file_reader::SimpleFileReader, wrapper::Wrapper};
use polywrap_wasm::wasm_wrapper::WasmWrapper;

use crate::{error::FFIError, invoker::FFIInvoker, wrapper::FFIAbortHandlerWrapping};

pub struct FFIWasmWrapper {
    pub inner_wasm_wrapper: Arc<dyn Wrapper>,
}

impl FFIWasmWrapper {
    pub fn new(wasm_module: Vec<u8>) -> FFIWasmWrapper {
        let wasm_wrapper = WasmWrapper::new(wasm_module, Arc::new(SimpleFileReader::new()));
        FFIWasmWrapper {
            inner_wasm_wrapper: Arc::new(wasm_wrapper),
        }
    }

    pub fn invoke(
        &self,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
        abort_handler: Option<Arc<FFIAbortHandlerWrapping>>,
    ) -> Result<Vec<u8>, FFIError> {
        let abort_handler = abort_handler.map(|a| {
            Box::new(move |msg: String| a.0.abort(msg)) as Box<dyn Fn(String) + Send + Sync>
        });

        Ok(self.inner_wasm_wrapper.invoke(
            method,
            args.as_deref(),
            env.as_deref(),
            invoker.0.clone(),
            abort_handler,
        )?)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_client::msgpack::decode;
    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_wrapper};

    use crate::invoker::FFIInvoker;

    use super::FFIWasmWrapper;

    #[test]
    fn ffi_invoke() {
        let wrapper = get_mock_wrapper();
        let ffi_wrapper = FFIWasmWrapper {
            inner_wasm_wrapper: wrapper,
        };

        let ffi_invoker = Arc::new(FFIInvoker(get_mock_invoker()));

        let response = ffi_wrapper
            .invoke("foo", None, None, ffi_invoker, None)
            .unwrap();
        assert!(decode::<bool>(&response).unwrap());
    }
}
