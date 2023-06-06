use std::path::{Path, PathBuf};

pub fn get_tests_path() -> Result<PathBuf, ()> {
    let path = Path::new("../../packages/tests-utils/cases")
        .canonicalize()
        .unwrap();
    Ok(path)
}
