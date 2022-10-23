use crate::error::WrapperError;
use crate::file_reader::FileReader;
use crate::wasm_instance::State;
use polywrap_core::error::CoreError;
use polywrap_core::invoke::InvokeArgs;
use polywrap_core::invoke::InvokeOptions;
use polywrap_core::invoke::Invoker;
use polywrap_core::wrapper::Encoding;
use polywrap_core::wrapper::GetFileOptions;
use polywrap_core::wrapper::Wrapper;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::sync::Mutex;

use crate::wasm_instance::WasmInstance;
use crate::wasm_instance::WasmModule;
use wasmtime::Val;

pub struct WasmWrapper {
    wasm_module: Option<WasmModule>,
    file_reader: Box<dyn FileReader>,
}

pub struct WasmWrapperConfig {
    pub file_reader: Box<dyn FileReader>,
}

impl WasmWrapper {
    pub fn new(config: WasmWrapperConfig) -> Self {
        Self {
            wasm_module: None,
            file_reader: config.file_reader,
        }
    }

    fn get_wasm_module(&self) -> Result<&WasmModule, WrapperError> {
        if self.wasm_module.is_none() {
            let file_content = self.file_reader.read_file("test/wrap.wasm");

            match file_content {
                Ok(content) => {
                    drop(content);
                }
                Err(err) => {
                    return Err(WrapperError::FileReadError(err));
                }
            }
        }

        Ok(self.wasm_module.as_ref().unwrap())
    }

    pub fn invoke_and_decode<T: DeserializeOwned>(
        &self,
        options: &InvokeOptions,
        invoker: Arc<Mutex<dyn Invoker>>,
    ) -> Result<T, CoreError> {
        let result = self.invoke(options, invoker)?;

        rmp_serde::from_slice(result.as_slice()).map_err(|e| CoreError::WrapperError(e.to_string()))
    }
}

impl Wrapper for WasmWrapper {
    fn invoke(
        &self,
        options: &InvokeOptions,
        invoker: Arc<Mutex<dyn Invoker>>,
    ) -> Result<Vec<u8>, CoreError> {
        let args = match options.args {
          Some(args) => match args {
            InvokeArgs::Values(values) => rmp_serde::encode::to_vec(&values).unwrap(),
            InvokeArgs::UIntArray(arr) => arr.clone(),
          },
          None => vec![],
        };

        let state = State::new(
            options.method,
            args.clone(),
        );

        let params = &[
            Val::I32(state.method.len().try_into().unwrap()),
            Val::I32(state.args.len().try_into().unwrap()),
            Val::I32(1),
        ];

        let state = Arc::new(Mutex::new(state));
        let abort_uri = options.uri.clone();
        let abort_method = options.method.to_string();
        let abort_args = args.clone();

        let abort = Arc::new(move |msg| {
            panic!(
                r#"WasmWrapper: Wasm module aborted execution.
              URI: {uri}
              Method: {method}
              Args: {args:?}
              Message: {message}.
            "#,
                uri = abort_uri,
                method = abort_method,
                args = abort_args,
                message = msg
            );
        });

        let wasm_module = self
            .get_wasm_module()
            .map_err(|e| CoreError::WrapperError(e.to_string()))?;

        let mut wasm_instance =
            WasmInstance::new(wasm_module, Arc::clone(&state), abort.clone(), invoker).unwrap();

        let mut result: [Val; 1] = [Val::I32(0)];
        wasm_instance
            .call_export("_wrap_invoke", params, &mut result)
            .map_err(|e| CoreError::WrapperError(e.to_string()))?;

        let state_guard = state.lock().unwrap();

        if result[0].unwrap_i32() == 1 {
            if state_guard.invoke.result.is_none() {
                abort("Invoke result is missing".to_string());
            }

            Ok(state_guard.invoke.result.as_ref().unwrap().to_vec())
        } else {
            if state_guard.invoke.error.as_ref().is_none() {
                abort("Invoke error is missing".to_string());
            }

            Err(CoreError::WrapperError(
                state_guard.invoke.error.as_ref().unwrap().to_string(),
            ))
        }
    }

    fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, CoreError> {
        let data_result = self.file_reader.read_file(&options.path);

        if data_result.is_err() {
            return Err(CoreError::WrapperError(format!(
                "WasmWrapper: File was not found.\nSubpath: {}",
                options.path
            )));
        };

        let data = data_result.unwrap();

        let result = match &options.encoding {
            Some(encoding) => {
                let data_string = String::from_utf8(data.clone()).unwrap();

                match encoding {
                    Encoding::Base64 => base64::decode(&data_string).unwrap(),
                    Encoding::UTF8 => data,
                }
            }
            None => data,
        };

        Ok(result)
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn invoke_test() {
//         let mut wasm_wrapper =
//             crate::wasm_wrapper::WasmWrapper::new(crate::wasm_wrapper::WasmWrapperConfig {
//                 file_reader: todo!(),
//             });

//         let invoke_result = wasm_wrapper.invoke::<String>(
//             "method",
//             &vec![
//                 130, 164, 97, 114, 103, 49, 179, 49, 50, 51, 52, 46, 53, 54, 55, 56, 57, 49, 50,
//                 51, 52, 53, 54, 55, 56, 57, 163, 111, 98, 106, 129, 165, 112, 114, 111, 112, 49,
//                 179, 57, 56, 46, 55, 54, 53, 52, 51, 50, 49, 57, 56, 55, 54, 53, 52, 51, 50, 49,
//             ],
//         );

//         assert_eq!(
//             invoke_result.unwrap(),
//             "121932.631356500531347203169112635269"
//         );
//     }
// }
