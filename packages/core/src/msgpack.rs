use serde::de::DeserializeOwned;

pub trait Decoder {
  fn decode<T: DeserializeOwned>(&self, data: Vec<u8>) -> Result<T, CoreError>;
}

pub trait Encoder {
  fn encode<T: Serialize>(&self, data: T) -> Result<Vec<u8>, CoreError>;
}