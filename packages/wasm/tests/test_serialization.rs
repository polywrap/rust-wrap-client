use std::{fs, path::Path};

use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_module::WasmModule;

#[test]
fn wasm_module_serialization() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();

    let module = WasmModule::WasmBytecode(module_bytes);

    let module = module.compile().unwrap();

    let result = module.serialize().unwrap();

    let result = result.deserialize();

    assert!(result.is_ok());
}
