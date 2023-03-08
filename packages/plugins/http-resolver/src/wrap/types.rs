#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use serde::{Serialize, Deserialize};
use num_bigint::BigInt;
use bigdecimal::BigDecimal as BigNumber;
use serde_json as JSON;
use std::collections::BTreeMap as Map;
use std::sync::Arc;
use polywrap_msgpack::{decode, serialize};
use polywrap_core::{invoke::{Invoker}, uri::Uri};
use polywrap_plugin::error::PluginError;

// Env START //

// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
    pub uri: Option<String>,
    pub manifest: Option<Vec<u8>>,
}
// Objects END //

// Enums START //

// Enums END //

// Imported objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpRequest {
    pub headers: Option<Map<String, String>>,
    pub url_params: Option<Map<String, String>>,
    pub response_type: HttpResponseType,
    pub body: Option<String>,
    pub form_data: Option<Vec<HttpFormDataEntry>>,
    pub timeout: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpFormDataEntry {
    pub name: String,
    pub value: Option<String>,
    pub file_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpResponse {
    pub status: i32,
    pub status_text: String,
    pub headers: Option<Map<String, String>>,
    pub body: Option<String>,
}
// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HttpResponseType {
    TEXT,
    BINARY,
    _MAX_
}
// Imported enums END //

// Imported Modules START //

// URI: "ens/http.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpModuleArgsGet {
    pub url: String,
    pub request: Option<HttpRequest>,
}

// URI: "ens/http.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpModuleArgsPost {
    pub url: String,
    pub request: Option<HttpRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpModule {}

impl HttpModule {
    pub const URI: &'static str = "ens/http.polywrap.eth";

    pub fn new() -> HttpModule {
        HttpModule {}
    }

    pub fn get(args: &HttpModuleArgsGet, invoker: Arc<dyn Invoker>) -> Result<Option<HttpResponse>, PluginError> {
        let uri = HttpModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "get",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "get".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(Some(decode(result.as_slice())?))
    }

    pub fn post(args: &HttpModuleArgsPost, invoker: Arc<dyn Invoker>) -> Result<Option<HttpResponse>, PluginError> {
        let uri = HttpModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "post",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "post".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(Some(decode(result.as_slice())?))
    }
}
// Imported Modules END //
