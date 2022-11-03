use crate::AnyManifest;

pub mod from_010_to_020;

#[derive(Clone)]
pub struct Migrator {
    pub from: String,
    pub to: String,
    pub migrate: fn(AnyManifest) -> AnyManifest,
}
