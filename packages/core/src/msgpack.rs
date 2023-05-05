use serde::de::DeserializeOwned;

pub trait Decoder {
  fn decode<T: DeserializeOwned>(&self, data: &[u8]) -> Result<T, Error>;
}

pub trait Encoder {
  fn encode<T: Serialize>(&self, data: &T) -> Result<Vec<u8>, Error>;
}