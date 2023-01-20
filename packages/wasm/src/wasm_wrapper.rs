use crate::error::WrapperError;
use crate::wasm_runtime::instance::State;
use async_trait::async_trait;
use polywrap_core::env::Env;
use polywrap_core::error::Error;
use polywrap_core::file_reader::FileReader;
use polywrap_core::invoke::Invoker;
use polywrap_core::resolvers::uri_resolution_context::UriResolutionContext;
use polywrap_core::uri::Uri;
use polywrap_core::wrapper::Encoding;
use polywrap_core::wrapper::GetFileOptions;
use polywrap_core::wrapper::Wrapper;
use wrap_manifest_schemas::versions::WrapManifest;
use polywrap_msgpack::decode;
use serde::de::DeserializeOwned;
use std::fmt::Formatter;
use std::{sync::Arc, fmt::Debug};

use crate::wasm_runtime::instance::WasmInstance;
use wasmtime::Val;

#[derive(Clone)]
pub struct WasmWrapper {
    wasm_module: Vec<u8>,
    file_reader: Arc<dyn FileReader>,
    manifest: WrapManifest,
}

impl WasmWrapper {
    pub fn new(
        wasm_module: Vec<u8>,
        file_reader: Arc<dyn FileReader>,
        manifest: WrapManifest,
    ) -> Self {
        Self {
            wasm_module,
            file_reader,
            manifest,
        }
    }

    pub fn get_wasm_module(&self) -> Result<&[u8], WrapperError> {
        Ok(&self.wasm_module)
    }

    pub fn get_manifest(&self) -> Result<&WrapManifest, WrapperError> {
        Ok(&self.manifest)
    }

    pub async fn invoke_and_decode<T: DeserializeOwned>(
        &mut self,
        invoker: Arc<dyn Invoker>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        resolution_context: Option<&mut UriResolutionContext>,
        env: Option<Env>,
    ) -> Result<T, Error> {
        let result = self
            .invoke(invoker, uri, method, args, env, resolution_context)
            .await?;

        let result = decode(result.as_slice())?;

        Ok(result)
    }
}

impl PartialEq for WasmWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.get_wasm_module().unwrap() == other.get_wasm_module().unwrap()
    }
}

impl Debug for WasmWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "WasmWrapper: {:?}", self)
    }
}

#[async_trait]
impl Wrapper for WasmWrapper {
    async fn invoke(
        &mut self,
        invoker: Arc<dyn Invoker>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let args = match args {
            Some(args) => args.to_vec(),
            None => vec![],
        };

        let env = match env {
            Some(e) => polywrap_msgpack::serialize(&e)?,
            None => polywrap_msgpack::encode(&rmpv::Value::Nil)?,
        };

        let params = &[
            Val::I32(method.to_string().len().try_into().unwrap()),
            Val::I32(args.len().try_into().unwrap()),
            Val::I32(env.len().try_into().unwrap()),
        ];

        let abort_uri = uri.clone();
        let abort_method = method.to_string();
        let abort_args = args.clone();
        let abort_env = env.clone();

        let abort = Box::new(move |msg| {
            panic!(
                r#"WasmWrapper: Wasm module aborted execution.
              URI: {uri}
              Method: {method}
              Args: {args:?}
              Env: {env:?}
              Message: {message}.
            "#,
                uri = abort_uri,
                method = abort_method,
                args = abort_args,
                env = abort_env,
                message = msg
            );
        });

        let state = State::new(invoker, abort.clone(), method, args, env);
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

    async fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error> {
        if let Ok(data) = self.file_reader.read_file(&options.path).await {
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
