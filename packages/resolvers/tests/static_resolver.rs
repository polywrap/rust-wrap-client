use polywrap_resolvers::{
  static_resolver::StaticResolver,
};
use polywrap_core::{
  file_reader::SimpleFileReader,
  uri_resolution_context::{UriPackage,UriWrapper},
  uri::Uri
};
use polywra_wasm::{
  wasm_package::WasmPackage
};

#[tokio:test]
async fn static_resolver_test() {
  let file_reader = SimpleFileReader::new();
  let package = WasmPackage(file_reader, None, None);

  let uri_package = UriPackage {
    uri: Uri::new("ens/package.eth"),
    package
  };

  let resolver = StaticResolver::from([
    uri_package
  ]);
  
  let result = resolver.try_resolve_uri().await.unwrap();
  dbg!(result);
}