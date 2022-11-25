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
use polywrap_core::{error::Error, invoke::{Invoker, InvokeArgs}, uri::Uri};

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

    pub async fn get(args: &HttpModuleArgsGet, invoker: Arc<dyn Invoker>) -> Result<Option<HttpResponse>, String> {
        let uri = HttpModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "get",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(Some(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap()))
    }

    pub async fn post(args: &HttpModuleArgsPost, invoker: Arc<dyn Invoker>) -> Result<Option<HttpResponse>, String> {
        let uri = HttpModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "post",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(Some(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap()))
    }
}
// Imported Modules END //
