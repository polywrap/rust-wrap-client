use std::{sync::Arc, fmt::Debug};

use async_trait::async_trait;
use wrap_manifest_schemas::{versions::WrapManifest};
use futures::lock::Mutex;

use crate::{error::Error, wrapper::Wrapper};

pub struct GetManifestOptions {
    pub no_validate: bool,
}

pub struct SerializeManifestOptions {
    pub no_validate: bool,
}

#[async_trait]
pub trait WrapPackage: Send + Sync + Debug {
    async fn create_wrapper(
        &self,
    ) -> Result<Arc<Mutex<dyn Wrapper>>, Error>;
    async fn get_manifest(
        &self,
        options: Option<GetManifestOptions>,
    ) -> Result<WrapManifest, Error>;
}
