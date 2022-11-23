use std::collections::HashMap;

pub use rmpv;
pub use rmpv::{encode::write_value, Value};
pub use rmp_serde::{Serializer};
pub use serde::{Serialize, de::DeserializeOwned, Deserialize};

pub fn encode(value: &rmpv::Value) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    let mut buf = Vec::new();
    write_value(&mut buf, value)?;
    Ok(buf)
}

pub fn serialize<T: Serialize>(val: T) -> Result<Vec<u8>, rmp_serde::encode::Error> {
  let mut buf = Vec::new();
  val.serialize(&mut Serializer::new(&mut buf))?;
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

#[derive(Deserialize,Debug,PartialEq)]
struct ValueTest {
    pub code: i32,
    pub success: bool,
    pub payload: HashMap<String, Vec<String>>
}

#[test]
fn vec_to_struct() {
    let encoded_struct: [u8; 50] = [
        131, 164, 99, 111, 100, 101, 204, 200, 167, 115, 117, 99, 99, 101, 115, 115, 195,
        167, 112, 97, 121, 108, 111, 97, 100, 129, 168, 102, 101, 97, 116, 117, 114, 101,
        115, 146, 165, 115, 101, 114, 100, 101, 167, 109, 115, 103, 112, 97, 99, 107
    ];

    let response = decode::<ValueTest>(&encoded_struct);
    let mut payload = HashMap::new();
    payload.insert("features".to_string(), vec!["serde".to_string(), "msgpack".to_string()]);
    let expected = ValueTest {
        code: 200,
        success: true,
        payload
    };
    assert_eq!(response.unwrap(), expected);
}