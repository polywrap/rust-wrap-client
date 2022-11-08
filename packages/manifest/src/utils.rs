use std::str::Split;

pub fn sanitize_semver_version(version_str: &str) -> String {
    let split: Split<&str> = version_str.split(".");
    if split.count() == 2 {
        format!("{}.0", version_str)
    } else {
        version_str.to_string()
    }
}