use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use polywrap_core::{package::WrapPackage, wrapper::Wrapper};
use wrap_manifest_schemas::versions::WrapManifest;

use super::{DifferentMockWrapper, MockWrapper};

pub struct MockPackage;
pub struct DifferentMockPackage;

impl Debug for MockPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockPackage")
    }
}

impl Debug for DifferentMockPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DifferentMockPackage")
    }
}

impl WrapPackage for MockPackage {
    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, polywrap_core::error::Error> {
        Ok(Arc::new(MockWrapper {}))
    }

    fn get_manifest(
        &self,
        _: Option<&polywrap_core::package::GetManifestOptions>,
    ) -> Result<WrapManifest, polywrap_core::error::Error> {
        unimplemented!()
    }
}

impl WrapPackage for DifferentMockPackage {
    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, polywrap_core::error::Error> {
        Ok(Arc::new(DifferentMockWrapper {}))
    }

    fn get_manifest(
        &self,
        _: Option<&polywrap_core::package::GetManifestOptions>,
    ) -> Result<WrapManifest, polywrap_core::error::Error> {
        unimplemented!()
    }
}

pub fn get_mock_package() -> Arc<dyn WrapPackage> {
    Arc::new(MockPackage {})
}

pub fn get_different_mock_package() -> Arc<dyn WrapPackage> {
    Arc::new(DifferentMockPackage {})
}
