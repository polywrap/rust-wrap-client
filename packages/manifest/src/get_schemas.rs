use std::collections::HashMap;
use serde_json::Value;

pub fn get_schemas() -> Result<HashMap<String, Value>, polywrap_core::error::Error> {
  Ok(HashMap::from([
    (
        "0.1.0".to_string(),
        serde_json::from_str::<Value>(include_str!("../schemas/0.1.0.json"))
            .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?,
    ),
    (
        "0.2.0".to_string(),
        serde_json::from_str::<Value>(include_str!("../schemas/0.2.0.json"))
            .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?,
    ),
]))
}