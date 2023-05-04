use std::{path::{Path,PathBuf},sync::{Arc}, fmt::{Debug, Formatter}};

use polywrap_core::{wrapper::{Wrapper, GetFileOptions},  invoke::Invoker, uri::Uri, env::Env, resolvers::uri_resolution_context::UriResolutionContext, package::WrapPackage};
use wrap_manifest_schemas::versions::WrapManifest;

pub struct MockWrapper {
    pub name: String
}
pub struct MockPackage {
    pub name: String
}

impl Debug for MockWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockWrapper")
    }
}

impl MockWrapper {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or("MockWrapper".to_string())
        }
    }
}


impl Debug for MockPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockPackage")
    }
}

impl MockPackage {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or("MockWrapper".to_string())
        }
    }
}

impl WrapPackage for MockPackage {
    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, polywrap_core::error::Error> {
        Ok(Arc::new(MockWrapper::new(None)))
    }

    fn get_manifest(
        &self, 
        _: Option<polywrap_core::package::GetManifestOptions>
    ) ->  Result<WrapManifest, polywrap_core::error::Error> {
        unimplemented!()
    }
}

impl Wrapper for MockWrapper {
    fn invoke(
        &self,
        _: Arc<dyn Invoker>,
        _: &Uri,
        _: &str,
        _: Option< &[u8]>,
        _: Option<&Env>,
        _: Option<&mut UriResolutionContext>
    ) ->  Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![2])
    }

    fn get_file(
        &self,
        _: &GetFileOptions
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![2])
    }
}

pub fn get_mock_package(name: Option<String>) -> Arc<dyn WrapPackage> {
    Arc::new(MockPackage::new(name))
}

pub fn get_mock_wrapper(name: Option<String>) -> Arc<dyn Wrapper> {
    Arc::new(MockWrapper::new(name))
}

pub fn get_tests_path() -> Result<PathBuf, ()> {
    let path = Path::new("../../packages/tests-utils/cases").canonicalize().unwrap();
    Ok(path)
}