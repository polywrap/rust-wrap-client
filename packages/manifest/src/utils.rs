pub fn sanitize_semver_version(version_str: &str) -> String {
    let split: Vec<&str> = version_str.split('.').collect();
    if split.len() == 2 {
        format!("{}.0", version_str)
    } else {
        version_str.to_string()
    }
}