use std::sync::Arc;

use polywrap_client::core::{file_reader::SimpleFileReader, wrapper::Wrapper};
use polywrap_wasm::{wasm_wrapper::WasmWrapper, wasm_module::{CompiledWasmModule, SerializedWasmModule}};

use crate::{error::FFIError, invoker::FFIInvoker};

pub fn ffi_wasm_wrapper_from_bytecode(
  bytes: &[u8]
) -> Result<FFIWasmWrapper, FFIError> {
    let wasm_module = CompiledWasmModule::try_from_bytecode(bytes)?;

    Ok(FFIWasmWrapper::new(
      Arc::new(FFICompiledWasmModule(Arc::new(wasm_module)))
    ))
}

pub fn ffi_compiled_wasm_module_from_bytecode(
  bytes: &[u8]
) -> Result<FFICompiledWasmModule, FFIError> {
  Ok(FFICompiledWasmModule(Arc::new(CompiledWasmModule::try_from_bytecode(bytes)?)))
}

pub struct FFISerializedWasmModule(Arc<SerializedWasmModule>);

impl FFISerializedWasmModule {
  pub fn deserialize(&self) -> Result<Arc<FFICompiledWasmModule>, FFIError> {
    let deserialized = self.0.deserialize()?;

    Ok(Arc::new(FFICompiledWasmModule(Arc::new(deserialized))))
  }
}

pub struct FFICompiledWasmModule(Arc<CompiledWasmModule>);

impl FFICompiledWasmModule {
    pub fn serialize(&self) -> Result<Arc<FFISerializedWasmModule>, FFIError> {
      let serialized = self.0.serialize()?;

      Ok(Arc::new(FFISerializedWasmModule(Arc::new(serialized))))
    }
}

pub struct FFIWasmWrapper {
    pub inner_wasm_wrapper: Arc<dyn Wrapper>,
}

impl FFIWasmWrapper {
    pub fn new(compiled_wasm_module: Arc<FFICompiledWasmModule>) -> FFIWasmWrapper {
        let compiled_wasm_module = compiled_wasm_module.as_ref().0.as_ref().clone();
        let wasm_wrapper =
            WasmWrapper::new(compiled_wasm_module, Arc::new(SimpleFileReader::new()));
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
    ) -> Result<Vec<u8>, FFIError> {
        Ok(self.inner_wasm_wrapper.invoke(
            method,
            args.as_deref(),
            env.as_deref(),
            invoker.0.clone(),
        )?)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_msgpack_serde::from_slice;
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
            .invoke("foo", None, None, ffi_invoker)
            .unwrap();
        assert!(from_slice::<bool>(&response).unwrap());
    }
}
