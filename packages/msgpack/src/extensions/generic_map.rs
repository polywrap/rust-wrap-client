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