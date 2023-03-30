use std::{collections::{BTreeMap, HashMap}, marker::PhantomData};

use rmp_serde::{to_vec, from_slice};
use serde::{de::{Unexpected, DeserializeOwned}, Serialize};
use serde_bytes::ByteBuf;
use serde_json::Map;

#[derive(Debug, PartialEq, Clone)]
pub struct GenericMap<K, V>(pub BTreeMap<K, V>);

struct GenericMapVisitor<K: Ord, V> {
  marker: PhantomData<fn() -> GenericMap<K, V>>
}

impl<K: Ord, V> GenericMapVisitor<K, V> {
  fn new() -> Self {
    GenericMapVisitor {
          marker: PhantomData
      }
  }
}

impl<K: Ord, V> Serialize for GenericMap<K, V>
where K: Serialize, V: Serialize, {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where S: serde::ser::Serializer
    {
        let value = {
            let tag = 1_i8;
            let encoded_map = to_vec(&self.0).unwrap();
            let byte_buf = ByteBuf::from(encoded_map);

            (tag, byte_buf)
        };
        s.serialize_newtype_struct(rmpv::MSGPACK_EXT_STRUCT_NAME, &value)
    }
}

impl<'de, K, V> serde::de::Visitor<'de> for GenericMapVisitor<K, V> where
K: DeserializeOwned + Ord,
V: DeserializeOwned, {
    type Value = GenericMap<K, V>;

    fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "a sequence of tag & binary")
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: serde::de::SeqAccess<'de>
    {
        let tag: i8 = seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let data: ByteBuf = seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

        if tag == 1 {
            Ok(GenericMap(from_slice(&data).unwrap()))
        } else {
            let unexp = Unexpected::Signed(tag as i64);
            Err(serde::de::Error::invalid_value(unexp, &self))
        }
    }
}

impl<'de, K, V> serde::de::Deserialize<'de> for GenericMap<K, V> where
K: DeserializeOwned + Ord,
V: DeserializeOwned, {
    fn deserialize<D>(deserializer: D) -> Result<GenericMap<K, V>, D::Error>
        where D: serde::Deserializer<'de>,
    {
        let visitor = GenericMapVisitor::new();
        deserializer.deserialize_newtype_struct(rmpv::MSGPACK_EXT_STRUCT_NAME, visitor)
    }
}


// TODO: HACK to get around msgpack encoding in Swift
pub fn convert_msgpack_to_json(value: rmpv::Value) -> serde_json::Value {
  match value {
    rmpv::Value::Nil => serde_json::Value::Null,
    rmpv::Value::Boolean(v) => serde_json::Value::Bool(v),
    rmpv::Value::Integer(v) => serde_json::from_str(&v.to_string()).unwrap(),
    rmpv::Value::F32(v) => serde_json::from_str(&v.to_string()).unwrap(),
    rmpv::Value::F64(v) => serde_json::from_str(&v.to_string()).unwrap(),
    rmpv::Value::String(v) => serde_json::from_str(&v.to_string()).unwrap(),
    rmpv::Value::Binary(v) => serde_json::Value::Array(v.into_iter().map(|v| serde_json::Value::Number(v.into())).collect()),
    rmpv::Value::Array(arr) => {
      let mut new_arr: Vec<serde_json::Value> = vec![];
      for val in arr.iter() {
        new_arr.push(convert_msgpack_to_json(val.clone()))
      }
      serde_json::Value::Array(new_arr)
    },
    rmpv::Value::Map(map) => {
      let mut new_map = serde_json::Map::new();
      for (key, value) in map.iter() {
          new_map.insert(key.as_str().unwrap().to_string(), convert_msgpack_to_json(value.clone()));
      }
      serde_json::Value::Object(new_map)
    },
    rmpv::Value::Ext(tag, data) => {
      rmp_serde::from_slice(&data).unwrap()
    },
}
}