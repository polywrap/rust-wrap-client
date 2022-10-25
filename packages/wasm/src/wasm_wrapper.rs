// use crate::error::WrapperError;
// use crate::file_reader::FileReader;
use crate::wasm_runtime::instance::State;
use async_trait::async_trait;
use polywrap_core::error::Error;
use polywrap_core::invoke::InvokeArgs;
use polywrap_core::invoke::InvokeOptions;
use polywrap_core::invoke::Invoker;
// use polywrap_core::wrapper::Encoding;
// use polywrap_core::wrapper::GetFileOptions;
use polywrap_core::wrapper::Wrapper;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::sync::Mutex;

use crate::wasm_runtime::instance::WasmInstance;
use crate::wasm_runtime::instance::WasmModule;
use wasmtime::Val;

pub struct WasmWrapper {
    wasm_module: WasmModule,
    // file_reader: Box<dyn FileReader>,
}

pub struct WasmWrapperConfig {
    pub wasm_module: WasmModule,
    // pub file_reader: Box<dyn FileReader>,
}

impl WasmWrapper {
    pub fn new(config: WasmWrapperConfig) -> Self {
        Self {
            wasm_module: config.wasm_module,
            // file_reader: config.file_reader,
        }
    }

    // fn get_wasm_module(&self) -> Result<&WasmModule, WrapperError> {
    //     if self.wasm_module.is_none() {
    //         let file_content = self.file_reader.read_file("test/wrap.wasm");

    //         match file_content {
    //             Ok(content) => {
    //                 drop(content);
    //             }
    //             Err(err) => {
    //                 return Err(WrapperError::FileReadError(err));
    //             }
    //         }
    //     }

    //     Ok(self.wasm_module.as_ref().unwrap())
    // }

    pub async fn invoke_and_decode<T: DeserializeOwned>(
        &self,
        options: &InvokeOptions<'_>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<T, Error> {
        let result = self.invoke(options, invoker).await?;

        rmp_serde::from_slice(result.as_slice()).map_err(|e| Error::WrapperError(e.to_string()))
    }
}

#[async_trait(?Send)]
impl Wrapper for WasmWrapper {
    async fn invoke(
        &self,
        options: &InvokeOptions,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, Error> {
        let args = match options.args {
            Some(args) => match args {
                InvokeArgs::JSON(json) => rmp_serde::encode::to_vec(&json).unwrap(),
                InvokeArgs::UIntArray(arr) => arr.clone(),
            },
            None => vec![],
        };

        let state = State::new(options.method, args.clone());

        let params = &[
            Val::I32(state.method.len().try_into().unwrap()),
            Val::I32(state.args.len().try_into().unwrap()),
            Val::I32(1),
        ];

        let state = Arc::new(Mutex::new(state));
        let abort_uri = options.uri.clone();
        let abort_method = options.method.to_string();
        let abort_args = args;

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

        let mut wasm_instance =
            WasmInstance::new(&self.wasm_module, Arc::clone(&state), abort.clone(), invoker).await.unwrap();

        let mut result: [Val; 1] = [Val::I32(0)];
        wasm_instance
            .call_export("_wrap_invoke", params, &mut result).await
            .map_err(|e| Error::WrapperError(e.to_string()))?;

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

            Err(Error::WrapperError(
                state_guard.invoke.error.as_ref().unwrap().to_string(),
            ))
        }
    }

    // fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error> {
    //     let data_result = self.file_reader.read_file(&options.path);

    //     if data_result.is_err() {
    //         return Err(Error::WrapperError(format!(
    //             "WasmWrapper: File was not found.\nSubpath: {}",
    //             options.path
    //         )));
    //     };

    //     let data = data_result.unwrap();

    //     let result = match &options.encoding {
    //         Some(encoding) => {
    //             let data_string = String::from_utf8(data.clone()).unwrap();

    //             match encoding {
    //                 Encoding::Base64 => base64::decode(&data_string).unwrap(),
    //                 Encoding::UTF8 => data,
    //             }
    //         }
    //         None => data,
    //     };

    //     Ok(result)
    // }
}
