/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use serde::{Serialize, Deserialize};
use num_bigint::BigInt;
use bigdecimal::BigDecimal as BigNumber;
use serde_json as JSON;
use std::collections::BTreeMap as Map;

// OBJECT
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub status: i32,
    pub status_text: String,
    pub headers: Option<Map<String, String>>,
    pub body: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub headers: Option<Map<String, String>>,
    pub url_params: Option<Map<String, String>>,
    pub response_type: ResponseType,
    pub body: Option<String>,
}

// ENV

// ENUM
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ResponseType {
    TEXT,
    BINARY,
    _MAX_
}

// Imported OBJECT

// Imported ENV

// Imported ENUM
