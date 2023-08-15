#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_plugin::*;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

pub type BigInt = String;

// Env START //

// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub status: i32,
    #[serde(rename = "statusText")]
    pub status_text: String,
    pub headers: Option<BTreeMap<String, String>>,
    pub body: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub headers: Option<BTreeMap<String, String>>,
    #[serde(rename = "urlParams")]
    pub url_params: Option<BTreeMap<String, String>>,
    #[serde(rename = "responseType")]
    pub response_type: ResponseType,
    pub body: Option<String>,
    #[serde(rename = "formData")]
    pub form_data: Option<Vec<FormDataEntry>>,
    pub timeout: Option<u32>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormDataEntry {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
// Objects END //

// Enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ResponseType {
    TEXT,
    BINARY,
    _MAX_
}
// Enums END //

// Imported objects START //

// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

// Imported enums END //

// Imported Modules START //

// Imported Modules END //
