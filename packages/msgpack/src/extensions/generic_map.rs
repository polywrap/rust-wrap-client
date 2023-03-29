use std::{collections::HashMap, marker::PhantomData, hash::Hash};

use rmp_serde::{to_vec, from_slice};
use serde::{de::{Unexpected, DeserializeOwned}, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, PartialEq)]
pub struct GenericMap<K: Hash + Eq, V>(pub HashMap<K, V>);

struct GenericMapVisitor<K: Hash + Eq, V> {
  marker: PhantomData<fn() -> GenericMap<K, V>>
}

impl<K: Hash + Eq, V> GenericMapVisitor<K, V> {
  fn new() -> Self {
    GenericMapVisitor {
          marker: PhantomData
      }
  }
}

impl<K: Hash + Eq, V> Serialize for GenericMap<K, V>
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
K: DeserializeOwned + Hash + Eq,
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

        let res: HashMap<K, V> = from_slice(&data).unwrap();

        if tag == 1 {
            Ok(GenericMap(res))
        } else {
            let unexp = Unexpected::Signed(tag as i64);
            Err(serde::de::Error::invalid_value(unexp, &self))
        }
    }
}

impl<'de, K, V> serde::de::Deserialize<'de> for GenericMap<K, V> where
K: DeserializeOwned + Hash + Eq,
V: DeserializeOwned, {
    fn deserialize<D>(deserializer: D) -> Result<GenericMap<K, V>, D::Error>
        where D: serde::Deserializer<'de>,
    {
        let visitor = GenericMapVisitor::new();
        deserializer.deserialize_newtype_struct(rmpv::MSGPACK_EXT_STRUCT_NAME, visitor)
    }
}