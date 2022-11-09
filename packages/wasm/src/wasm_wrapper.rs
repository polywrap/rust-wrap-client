use crate::error::WrapperError;
use crate::wasm_runtime::instance::State;
use async_trait::async_trait;
use polywrap_core::error::Error;
use polywrap_core::file_reader::FileReader;
use polywrap_core::invoke::InvokeArgs;
use polywrap_core::invoke::InvokeOptions;
use polywrap_core::invoke::Invoker;
use polywrap_core::wrapper::GetFileOptions;
use polywrap_core::wrapper::Encoding;
use polywrap_core::wrapper::Wrapper;
use polywrap_manifest::versions::WrapManifest;
use polywrap_msgpack::decode;
use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::wasm_runtime::instance::WasmInstance;
use crate::wasm_runtime::instance::WasmModule;
use wasmtime::Val;

pub struct WasmWrapper {
    wasm_module: WasmModule,
    file_reader: Arc<dyn FileReader>,
    manifest: WrapManifest,
}

impl WasmWrapper {
    pub fn new(
        wasm_module: WasmModule,
        file_reader: Arc<dyn FileReader>,
        manifest: WrapManifest,
    ) -> Self {
        Self {
            wasm_module,
            file_reader,
            manifest,
        }
    }

    pub fn get_wasm_module(&self) -> Result<&WasmModule, WrapperError> {
        Ok(&self.wasm_module)
    }

    pub fn get_manifest(&self) -> Result<&WrapManifest, WrapperError> {
        Ok(&self.manifest)
    }

    pub async fn invoke_and_decode<T: DeserializeOwned>(
        &self,
        options: &InvokeOptions<'_>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<T, Error> {
        let result = self.invoke(options, invoker).await?;

        decode(result.as_slice()).map_err(|e| Error::WrapperError(e.to_string()))
    }
}

#[async_trait]
impl Wrapper for WasmWrapper {
    async fn invoke(
        &self,
        options: &InvokeOptions,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, Error> {
        let args = match options.args {
            Some(args) => match args {
                InvokeArgs::Msgpack(value) => polywrap_msgpack::encode(value)
                    .map_err(|e| Error::MsgpackError(e.to_string()))?,
                InvokeArgs::UIntArray(arr) => arr.clone(),
            },
            None => vec![],
        };

        let params = &[
            Val::I32(options.method.to_string().len().try_into().unwrap()),
            Val::I32(args.len().try_into().unwrap()),
            Val::I32(1),
        ];

        let abort_uri = options.uri.clone();
        let abort_method = options.method.to_string();
        let abort_args = args.clone();

        let abort = Box::new(move |msg| {
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

        let state = State::new(invoker, abort.clone(), options.method, args);

        let mut wasm_instance = WasmInstance::new(&self.wasm_module, state).await.unwrap();

        let mut result: [Val; 1] = [Val::I32(0)];
        wasm_instance
            .call_export("_wrap_invoke", params, &mut result)
            .await
            .map_err(|e| Error::WrapperError(e.to_string()))?;

        let state = wasm_instance.store.data_mut();

        if result[0].unwrap_i32() == 1 {
            if state.invoke.result.is_none() {
                abort("Invoke result is missing".to_string());
            }

            Ok(state.invoke.result.as_ref().unwrap().to_vec())
        } else {
            if state.invoke.error.as_ref().is_none() {
                abort("Invoke error is missing".to_string());
            }

            Err(Error::WrapperError(
                state.invoke.error.as_ref().unwrap().to_string(),
            ))
        }
    }

    fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error> {
        if let Ok(data) = self.file_reader.read_file(&options.path) {
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
        } else {
            Err(Error::WrapperError(format!(
                "WasmWrapper: File was not found.\nSubpath: {}",
                options.path
            )))
        }
    }
}
