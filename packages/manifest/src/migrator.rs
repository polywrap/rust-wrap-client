use crate::formats::AnyManifest;

#[derive(Clone)]
pub struct Migrator {
    pub from: String,
    pub to: String,
    pub migrate: fn(AnyManifest) -> AnyManifest,
}
