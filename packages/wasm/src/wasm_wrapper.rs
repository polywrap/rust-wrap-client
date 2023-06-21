use crate::error::WrapperError;
use crate::runtime::instance::State;
use crate::wasm_module::CompiledWasmModule;

use polywrap_core::error::Error;
use polywrap_core::file_reader::FileReader;
use polywrap_core::invoker::Invoker;
use polywrap_core::wrapper::Encoding;
use polywrap_core::wrapper::GetFileOptions;
use polywrap_core::wrapper::Wrapper;
use polywrap_msgpack::{decode, msgpack};
use serde::de::DeserializeOwned;
use std::fmt::Formatter;
use std::sync::Mutex;
use std::{fmt::Debug, sync::Arc};
use wasmer::Value;

pub struct WasmWrapper {
    compiled_module: CompiledWasmModule,
    file_reader: Arc<dyn FileReader>,
}

impl WasmWrapper {
    pub fn new(compiled_module: CompiledWasmModule, file_reader: Arc<dyn FileReader>) -> Self {
        Self {
            compiled_module,
            file_reader,
        }
    }

    pub fn try_from_byte_code(bytes: &[u8], file_reader: Arc<dyn FileReader>) -> Result<Self, WrapperError> {
        let compiled_module = CompiledWasmModule::try_from_byte_code(bytes)?;
        
        Ok(Self {
            compiled_module,
            file_reader,
        })
    }

    pub fn invoke_and_decode<T: DeserializeOwned>(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
        abort_handler: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<T, Error> {
        let result = self.invoke(method, args, env, invoker, abort_handler)?;

        let result = decode(result.as_slice())?;

        Ok(result)
    }
}

impl Debug for WasmWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, r#"WasmModule(...)"#)
    }
}

impl Wrapper for WasmWrapper {
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
        abort_handler: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<Vec<u8>, Error> {
        let args = match args {
            Some(args) => args.to_vec(),
            None => msgpack!({}),
        };

        let env = match env {
            Some(env) => env.to_vec(),
            None => vec![],
        };

        let params = &[
            Value::I32(method.to_string().len().try_into().unwrap()),
            Value::I32(args.len().try_into().unwrap()),
            Value::I32(env.len().try_into().unwrap()),
        ];

        let abort_method = method.to_string();
        let abort_handler = Arc::new(abort_handler);

        let abort = Box::new(move |error_message: String| {
            if let Some(abort_handler) = abort_handler.as_ref() {
                // Use the abort handler if provided
                abort_handler(error_message);
            } else {
                // Otherwise, panic since this is an unrecoverable error
                panic!(
                    r#"WasmWrapper: Wasm module aborted execution.
                  Method: {abort_method}
                  Message: {error_message}.
                "#
                );
            }
        });

        let state = Arc::new(Mutex::new(State::new(
            invoker,
            abort.clone(),
            method,
            args,
            env,
        )));

        let mut wasm_instance = self.compiled_module.create_instance(state.clone())?;

        let result = wasm_instance
            .call_export("_wrap_invoke", params)
            .map_err(|e| Error::WrapperError(e.to_string()))?;

        let state = state.lock().unwrap();
        if result {
            if state.invoke.result.is_none() {
                return Err(Error::RuntimeError("Invoke result is missing".to_string()));
            }

            Ok(state.invoke.result.as_ref().unwrap().to_vec())
        } else if state.invoke.error.is_none() {
            Err(Error::RuntimeError("Invoke error is missing".to_string()))
        } else {
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
                        Encoding::Base64 => base64::decode(data_string).unwrap(),
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
