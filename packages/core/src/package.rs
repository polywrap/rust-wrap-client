use crate::{wrapper::Wrapper, error::Error};
use async_trait::async_trait;
use jsonschema::JSONSchema;

pub struct GetManifestOptions {
    pub no_validate: bool,
}

pub struct DeserializeManifestOptions {
    pub no_validate: bool,
    pub ext_schema: Option<JSONSchema>
}

pub struct SerializeManifestOptions {
    pub no_validate: bool,
}

#[async_trait]
pub trait WrapPackage: Send + Sync {
  async fn create_wrapper(&self, options: Option<DeserializeManifestOptions>) -> Result<Box<dyn Wrapper>, Error>;
  async fn get_manifest(&self, options: Option<GetManifestOptions>) -> Result<String, Error>;
}