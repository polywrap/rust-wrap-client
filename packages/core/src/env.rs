use std::collections::HashMap;

pub type Env = serde_json::Value;
// @TODO(cbrzn): Key should be Uri instead of String
pub type Envs = HashMap<String, Env>;