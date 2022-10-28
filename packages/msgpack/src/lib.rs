pub use rmpv::{encode::write_value, Value};
pub use rmp_serde::{Serializer};
pub use serde::{Serialize, de::DeserializeOwned};

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    let mut buf = Vec::new();
    value.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, rmp_serde::decode::Error> {
    rmp_serde::from_slice(bytes)
}

pub struct RMPVObject {
  pub values: Vec<(Value, Value)>
}

impl RMPVObject {
    pub fn new() -> Self {
        RMPVObject {
            values: Vec::new()
        }
    }

    pub fn insert(&mut self, key: Value, value: Value) {
        self.values.push((key, value));
    }

    pub fn get(&self, key: &Value) -> Option<&Value> {
        for (k, v) in &self.values {
            if k == key {
                return Some(v);
            }
        }
        None
    }
}

pub mod macros;