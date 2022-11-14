use std::collections::HashMap;
use polywrap_msgpack::Value;

pub type Env = Value;
pub type Envs = HashMap<String, Env>;