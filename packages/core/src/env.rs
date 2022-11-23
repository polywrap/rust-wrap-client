use std::collections::HashMap;

pub type Env = serde_json::Value;
pub type Envs = HashMap<String, Env>;