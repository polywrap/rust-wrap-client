use std::collections::HashMap;

use jsonschema::{JSONSchema, ValidationError};
use serde_json::Value;

use crate::AnyManifest;

pub fn validate_polywrap_manifest(
    manifest: AnyManifest,
    ext_schema: Option<JSONSchema>,
) -> Result<(), polywrap_core::error::Error> {
    let schemas = HashMap::from([
        (
            "0.1.0",
            serde_json::from_str::<Value>(include_str!("../schemas/0.1.0.json"))
                .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?,
        ),
        (
            "0.2.0",
            serde_json::from_str::<Value>(include_str!("../schemas/0.2.0.json"))
                .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?,
        ),
    ]);

    let panic_if_errors = |result: Result<
        (),
        Box<dyn Iterator<Item = ValidationError> + Send + Sync>,
    >| match result {
        Ok(_) => (),
        Err(e) => panic!(
            "Validation errors encountered while sanitizing PolywrapManifest format {}{}",
            manifest.format(),
            e.into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        ),
    };

    let manifest_schema = JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .compile(&schemas[manifest.format().as_str()])
        .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?;
    let manifest_json = manifest.to_json_value();
    panic_if_errors(manifest_schema.validate(&manifest_json));

    if ext_schema.is_some() {
        panic_if_errors(ext_schema.unwrap().validate(&manifest.to_json_value()));
    }

    Ok(())
}
