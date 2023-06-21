use std::{fs, path::Path};

use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_module::{CompiledWasmModule, WasmModule};

#[test]
fn compiled_wasm_module_from_bytecode() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();

    let result = CompiledWasmModule::try_from_bytecode(&module_bytes);

    assert!(result.is_ok());
}

#[test]
fn wasm_module_from_bytecode_compile() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();

    let module = WasmModule::WasmBytecode(module_bytes.into());

    let result = module.compile();

    assert!(result.is_ok());
}
