use std::path::{Path, PathBuf};

pub fn get_tests_path() -> Result<PathBuf, ()> {
    let path = Path::new("../../packages/tests-utils/cases")
        .canonicalize()
        .unwrap();
    Ok(path)
}

pub fn get_tests_path_string() -> String {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    path
}