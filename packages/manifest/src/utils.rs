use std::str::Split;

pub fn sanitize_semver_version(version_str: &str) -> String {
    let binding = String::from(".");
    let split: Split<&str> = version_str.split(&binding);
    if split.count() == 2 {
        format!("{version_str}.0")
    } else {
        version_str.to_string()
    }
}