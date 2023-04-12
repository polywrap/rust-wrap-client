use std::{collections::{BTreeMap}, marker::PhantomData};

use rmp_serde::{to_vec, from_slice};
use serde::{de::{Unexpected, DeserializeOwned}, Serialize};
use serde_bytes::ByteBuf;

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
      let tag = 1_i8;
      let encoded_map = to_vec(&self.0).unwrap();
      let byte_buf = ByteBuf::from(encoded_map);
      s.serialize_newtype_struct(rmpv::MSGPACK_EXT_STRUCT_NAME, &(tag, byte_buf))
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

        match tag {
          1 => Ok(GenericMap(from_slice(&data).unwrap())),
          _ => {
            let unexp = Unexpected::Signed(tag as i64);
            Err(serde::de::Error::invalid_value(unexp, &self))
          }
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

impl<'de, K, V> From<BTreeMap<K, V>> for GenericMap<K, V> where
K: DeserializeOwned + Ord,
V: DeserializeOwned, {
    fn from(value: BTreeMap<K, V>) -> Self {
        GenericMap(value)
    }
}

impl<'de, K, V> From<GenericMap<K, V>> for BTreeMap<K, V> where
K: DeserializeOwned + Ord,
V: DeserializeOwned, {
    fn from(value: GenericMap<K, V>) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use rmp_serde::{from_slice, to_vec};
    use super::GenericMap;

  #[test]
  fn from_msgpack_to_generic_map() {
    let mut map = BTreeMap::new();
    let mut map2 = BTreeMap::new();
    
    map2.insert("foo".to_string(), "bar".to_string());
    map.insert("key".to_string(), map2);
    
    let expected = GenericMap(map);
    let actual: GenericMap<String, BTreeMap<String, String>> = from_slice(&[199, 14, 1, 129, 163, 107, 101, 121, 129, 163, 102, 111, 111, 163, 98, 97, 114]).unwrap();

    assert_eq!(expected, actual)
  }

  #[test]
  fn from_generic_map_to_msgpack() {
    let mut map = BTreeMap::new();
    let mut map2 = BTreeMap::new();
    
    map2.insert("foo".to_string(), "bar".to_string());
    map.insert("key".to_string(), map2);
    
    let actual = to_vec(&GenericMap(map)).unwrap();
    assert_eq!(vec![199, 14, 1, 129, 163, 107, 101, 121, 129, 163, 102, 111, 111, 163, 98, 97, 114], actual)
  }

  #[test]
  fn from_msgpack_to_json() {
    let map: GenericMap<String, BTreeMap<String, String>> = from_slice(&[199, 14, 1, 129, 163, 107, 101, 121, 129, 163, 102, 111, 111, 163, 98, 97, 114]).unwrap();
    let json_str = serde_json::to_string(&map.0).unwrap();

    assert_eq!(json_str, "{\"key\":{\"foo\":\"bar\"}}")
  }

  #[test]
  fn from_json_to_msgpack() {
    let json_str = "{\"key\":{\"foo\":\"bar\"}}";
    let map: BTreeMap<String, BTreeMap<String, String>> = serde_json::from_str(&json_str).unwrap();
    let map: GenericMap<String, BTreeMap<String, String>> = GenericMap(map);

    let actual = to_vec(&map).unwrap();
    assert_eq!(vec![199, 14, 1, 129, 163, 107, 101, 121, 129, 163, 102, 111, 111, 163, 98, 97, 114], actual)
  }
}