use std::{
    fmt::{Debug, Formatter},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use polywrap_core::{
    invoker::Invoker,
    package::WrapPackage,
    wrapper::{GetFileOptions, Wrapper},
};
use wrap_manifest_schemas::versions::WrapManifest;

#[derive(Debug)]
pub struct MockWrapper;
#[derive(Debug)]
pub struct DifferentMockWrapper;

pub struct MockPackage;
pub struct DifferentMockPackage;

pub struct MockInvoker;

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

impl Wrapper for MockWrapper {
    fn invoke(
        &self,
        _: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Arc<dyn Invoker>,
        _: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![2])
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![2])
    }
}

impl Wrapper for DifferentMockWrapper {
    fn invoke(
        &self,
        _: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Arc<dyn Invoker>,
        _: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![1])
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![1])
    }
}

impl Invoker for MockInvoker {
    fn invoke_raw(
        &self,
        _: &polywrap_core::uri::Uri,
        _: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Option<Arc<Mutex<polywrap_core::resolution::uri_resolution_context::UriResolutionContext>>>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![3])
    }

    fn get_implementations(
        &self,
        _: &polywrap_core::uri::Uri,
    ) -> Result<Vec<polywrap_core::uri::Uri>, polywrap_core::error::Error> {
        Ok(vec![])
    }

    fn get_interfaces(
        &self,
    ) -> Option<polywrap_core::interface_implementation::InterfaceImplementations> {
        None
    }

    fn get_env_by_uri(&self, _: &polywrap_core::uri::Uri) -> Option<&[u8]> {
        None
    }
}

pub fn get_mock_package() -> Arc<dyn WrapPackage> {
    Arc::new(MockPackage {})
}

pub fn get_different_mock_package() -> Arc<dyn WrapPackage> {
    Arc::new(DifferentMockPackage {})
}

pub fn get_mock_wrapper() -> Arc<dyn Wrapper> {
    Arc::new(MockWrapper {})
}

pub fn get_mock_invoker() -> Arc<dyn Invoker> {
    Arc::new(MockInvoker {})
}

pub fn get_different_mock_wrapper() -> Arc<dyn Wrapper> {
    Arc::new(DifferentMockWrapper {})
}

pub fn get_tests_path() -> Result<PathBuf, ()> {
    let path = Path::new("../../packages/tests-utils/cases")
        .canonicalize()
        .unwrap();
    Ok(path)
}
