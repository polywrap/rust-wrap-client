use std::{fs,sync::Arc,path::Path};
use async_trait::async_trait;

use polywrap_resolvers::{
  static_resolver::StaticResolver,
  helpers::UriResolverLike
};
use polywrap_wasm::{wasm_runtime::instance::WasmModule};
use polywrap_core::{
  file_reader::SimpleFileReader,
  uri_resolution_context::{UriPackage,UriWrapper,UriResolutionContext, UriPackageOrWrapper},
  uri::Uri,
  uri_resolver::{UriResolver, UriResolverHandler},
  loader::Loader,
  wrapper::Wrapper,
  error::Error, 
  package::WrapPackage
};
use polywrap_manifest::{
  deserialize::{deserialize_wrap_manifest}
};
use polywrap_wasm::{
  wasm_package::WasmPackage, wasm_wrapper::WasmWrapper
};
use polywrap_tests::{
  helpers::get_tests_path
};

struct MockLoader {
  
}

impl MockLoader {
  fn new() -> MockLoader {
    Self {}
  }
}

#[async_trait]
impl Loader for MockLoader {
  async fn load_wrapper(
    &self, 
    uri: &Uri, 
    resolution_context: Option<&UriResolutionContext>
  ) -> Result<Box<dyn Wrapper>, Error> {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();let module_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.wasm", path);
    let module = WasmModule::Path(module_path);
    let file_reader = SimpleFileReader::new();
    let manifest_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.info", path);
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();
    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let wrapper = WasmWrapper::new(module, Arc::new(file_reader), manifest);
    Ok(Box::new(wrapper))
  }
}

#[async_trait]
impl UriResolverHandler for MockLoader {
  async fn try_resolve_uri(
    &self,
    uri: &Uri,
    resolution_context: Option<&mut UriResolutionContext>
  ) ->  Result<Arc<UriPackageOrWrapper>, Error> {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();let module_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.wasm", path);
    let module = WasmModule::Path(module_path);
    let file_reader = SimpleFileReader::new();
    let manifest_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.info", path);
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();
    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let wrapper = WasmWrapper::new(module, Arc::new(file_reader), manifest);
    let wrapper_package = UriWrapper {
      uri: Uri::new("ens/wrapper.eth"),
      wrapper: Box::new(wrapper)
    };
    let boxed = Arc::new(UriPackageOrWrapper::Wrapper(wrapper_package));  
    Ok(boxed)
  }
}

#[tokio::test]
async fn static_resolver_test() {
  let test_path = get_tests_path().unwrap();
  let path = test_path.into_os_string().into_string().unwrap();

  let file_reader = SimpleFileReader::new();
  let package = WasmPackage::new(Arc::new(file_reader), None, None);

  let module_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.wasm", path);
  let module = WasmModule::Path(module_path);
  let manifest_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.info", path);
  let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();
  let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
  let file_reader = SimpleFileReader::new();
  let wrapper = WasmWrapper::new(module, Arc::new(file_reader), manifest);
  
  let uri_package = UriPackage {
    uri: Uri::new("ens/package.eth"),
    package: Box::new(package)
  };
  
  let w = Box::new(wrapper);
  let mock_loader = MockLoader::new();
  let wrapper_package = UriWrapper {
    uri: Uri::new("ens/wrapper.eth"),
    wrapper: w
  };

  let mut resolution_context = UriResolutionContext::new();
  let package = UriResolverLike::Package(uri_package);
  let resolver = StaticResolver::_from(vec![package]);
  let result = resolver.try_resolve_uri(
    &Uri::new("ens/package.eth"),
    &mock_loader,
    &mut resolution_context
  ).await.unwrap();


  let t = result.as_ref();
  
  // if let Some(w) = result {

  // }

  // let wrapper = 
  // let uri_wrapper = UriWrapper {
  //   uri: Uri::new("ens/package.eth"),
  //   wrapper
  // };

  // let mut result: Option<Box<dyn WrapPackage>> = None;
  
  // if let UriPackageOrWrapper::Package(p) = t {
  //   result = Some(p.package);
  // }

  // let file_reader = SimpleFileReader::new();
  // let package = WasmPackage::new(Arc::new(file_reader), None, None);

  // assert_eq!(result, Some(Box::new(package)));
}