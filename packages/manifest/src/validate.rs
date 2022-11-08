use jsonschema::{JSONSchema, ValidationError};

use crate::{versions::AnyManifest, get_schemas::get_schemas};

pub fn validate_polywrap_manifest(
    manifest: &AnyManifest,
    ext_schema: Option<JSONSchema>,
) -> Result<(), super::error::Error> {
    let schemas = get_schemas().unwrap();

    let panic_if_errors = |result: Result<
        (),
        Box<dyn Iterator<Item = ValidationError> + Send + Sync>,
    >| match result {
        Ok(_) => (),
        Err(e) => panic!(
            "Validation errors encountered while sanitizing WrapManifest format {}{}",
            manifest.version(),
            e.into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        ),
    };

    let manifest_schema = JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .compile(&schemas[manifest.version().as_str()])?;
    let manifest_json = manifest.to_json_value()?;
    panic_if_errors(manifest_schema.validate(&manifest_json));

    if ext_schema.is_some() {
        panic_if_errors(ext_schema.unwrap().validate(&manifest.to_json_value()?));
    }

    Ok(())
}
