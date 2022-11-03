use polywrap_core::package::DeserializeManifestOptions;
use serde_json::Value;

use crate::{
    migrate::migrate_polywrap_manifest, validate::validate_polywrap_manifest, AnyManifest,
    PolywrapManifest, LATEST_MANIFEST_FORMAT,
};

pub fn deserialize_polywrap_manifest(
    manifest: &str,
    options: Option<DeserializeManifestOptions>,
) -> Result<PolywrapManifest, polywrap_core::error::Error> {
    let any_polywrap_manifest_json = match serde_json::from_str::<Value>(manifest) {
        Ok(any_polywrap_manifest) => any_polywrap_manifest,
        Err(_) => serde_yaml::from_str::<Value>(manifest)
            .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?,
    };

    let any_polywrap_manifest = AnyManifest::from_json_value(any_polywrap_manifest_json);

    match options {
        Some(opts) => {
            if opts.no_validate == false {
                validate_polywrap_manifest(any_polywrap_manifest.clone(), opts.ext_schema)?;
            };
        }
        None => validate_polywrap_manifest(any_polywrap_manifest.clone(), None)?,
    };

    let version_comparator = semver::Comparator::parse(&any_polywrap_manifest.format())
        .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?;

    let version_compare =
        version_comparator.matches(&semver::Version::parse(LATEST_MANIFEST_FORMAT).unwrap());

    if version_compare == false {
        return Ok(migrate_polywrap_manifest(
            any_polywrap_manifest,
            LATEST_MANIFEST_FORMAT.to_string(),
        ));
    } else {
        match any_polywrap_manifest {
            AnyManifest::PolywrapManifest020(m) => Ok(m),
            _ => panic!("Invalid manifest format"),
        }
    }
}
