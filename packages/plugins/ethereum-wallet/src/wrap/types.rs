#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_core::{invoker::Invoker, uri::Uri};
use polywrap_plugin::error::PluginError;
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  BigInt,
  BigNumber,
  JSON,
  bytes,
  wrappers::{
    polywrap_bigint as bigint,
    polywrap_json as json
  },
  JSONString
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

// Env START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Env {
    pub connection: Option<Connection>,
}
// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Connection {
    pub node: Option<String>,
    #[serde(rename = "networkNameOrChainId")]
    pub network_name_or_chain_id: Option<String>,
}
// Objects END //

// Enums START //

// Enums END //

// Imported objects START //

// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

// Imported enums END //

// Imported Modules START //

// Imported Modules END //
