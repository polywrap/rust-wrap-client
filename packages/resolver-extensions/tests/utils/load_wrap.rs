use std::fs;

pub fn load_wrap(path: &str) -> (Vec<u8>, Vec<u8>) {
  let manifest = fs::read(format!("{path}/wrap.info")).expect("Unable to read wrap manifest file");
  let module = fs::read(format!("{path}/wrap.wasm")).expect("Unable to read wrap module file");

  (manifest, module)
}