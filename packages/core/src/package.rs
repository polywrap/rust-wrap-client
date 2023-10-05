use std::{any::Any, fmt::Debug, sync::Arc};

use wrap_manifest_schemas::versions::WrapManifest;

use crate::{error::Error, wrapper::Wrapper};

/// Options for retrieving a package manifest.
pub struct GetManifestOptions {
    /// If set to true, the manifest will not be validated.
    pub no_validate: bool,
}

/// Options for serializing a package manifest.
pub struct SerializeManifestOptions {
    /// If set to true, the manifest will not be validated during serialization.
    pub no_validate: bool,
}

/// Defines a wrap package (wrap + manifest)
pub trait WrapPackage: Send + Sync + Debug + Any {
    /// Creates a `Wrapper` from the package.
    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, Error>;
    /// Retrieves the wrap's manifest.
    fn get_manifest(&self, options: Option<&GetManifestOptions>) -> Result<WrapManifest, Error>;
}
