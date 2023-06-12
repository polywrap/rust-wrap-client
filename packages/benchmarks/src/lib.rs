use polywrap_tests_utils::helpers::get_tests_path;

pub fn get_tests_path_string() -> String {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    path
}