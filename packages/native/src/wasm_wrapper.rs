use std::sync::Arc;

use polywrap_client::core::{file_reader::SimpleFileReader, wrapper::Wrapper};
use polywrap_wasm::{wasm_wrapper::WasmWrapper, wasm_module::{CompiledWasmModule, SerializedWasmModule}};

use crate::{error::FFIError, invoker::FFIInvoker};

pub fn ffi_wasm_wrapper_from_bytecode(
  bytes: &[u8]
) -> Result<Arc<FFIWasmWrapper>, FFIError> {
    let wasm_wrapper = WasmWrapper::try_from_bytecode(bytes, Arc::new(SimpleFileReader::new()))?;

    Ok(Arc::new(FFIWasmWrapper(Arc::new(wasm_wrapper))))
}

pub fn ffi_compiled_wasm_module_from_bytecode(
  bytes: &[u8]
) -> Result<Arc<FFICompiledWasmModule>, FFIError> {
  Ok(Arc::new(FFICompiledWasmModule(Arc::new(CompiledWasmModule::try_from_bytecode(bytes)?))))
}

pub struct FFISerializedWasmModule(pub Arc<SerializedWasmModule>);

impl FFISerializedWasmModule {
  pub fn deserialize(&self) -> Result<Arc<FFICompiledWasmModule>, FFIError> {
    let deserialized = self.0.clone().as_ref().clone().deserialize()?;

    Ok(Arc::new(FFICompiledWasmModule(Arc::new(deserialized))))
  }
}

pub struct FFICompiledWasmModule(pub Arc<CompiledWasmModule>);

impl FFICompiledWasmModule {
    pub fn serialize(&self) -> Result<Arc<FFISerializedWasmModule>, FFIError> {
      let serialized = self.0.serialize()?;

      Ok(Arc::new(FFISerializedWasmModule(Arc::new(serialized))))
    }
}

pub struct FFIWasmWrapper(pub Arc<dyn Wrapper>);

impl FFIWasmWrapper {
    pub fn new(compiled_wasm_module: Arc<FFICompiledWasmModule>) -> FFIWasmWrapper {
        let compiled_wasm_module = compiled_wasm_module.as_ref().0.as_ref().clone();
        let wasm_wrapper =
            WasmWrapper::new(compiled_wasm_module, Arc::new(SimpleFileReader::new()));
        FFIWasmWrapper(Arc::new(wasm_wrapper))
    }

    pub fn invoke(
        &self,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<Vec<u8>>,
        invoker: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError> {
        Ok(self.0.invoke(
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
        let ffi_wrapper = FFIWasmWrapper(wrapper);

        let ffi_invoker = Arc::new(FFIInvoker(get_mock_invoker()));

        let response = ffi_wrapper
            .invoke("foo", None, None, ffi_invoker)
            .unwrap();
        assert!(from_slice::<bool>(&response).unwrap());
    }
}
