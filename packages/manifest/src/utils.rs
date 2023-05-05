use std::str::Split;

pub fn sanitize_semver_version(version_str: &str) -> String {
    let binding = String::from(".");
    let split: Split<&str> = version_str.split(&binding);
    if split.count() == 2 {
        format!("{}.0", version_str)
    } else {
        version_str.to_string()
    }
}