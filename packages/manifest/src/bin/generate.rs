use std::{fs, io, path::PathBuf};

use handlebars::{handlebars_helper, Handlebars};
use polywrap_jsonref::JsonRef;
use semver::Version;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Schema {
    version: String,
}

#[derive(Serialize)]
struct VersionsData {
    versions: Vec<Schema>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error fetching Wrap jsonschemas through HTTP: `{0}`")]
    SchemaFetch(String),
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Error::SchemaFetch(e.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::SchemaFetch(e.to_string())
    }
}

handlebars_helper! {vsuffix: |v: str| {
  if v.contains('.') {
    v.to_string().split('.').map(|v| v.to_string()).collect::<Vec<String>>().join("")
  } else if v.contains('_') {
    v.to_string().split('_').map(|v| v.to_string()).collect::<Vec<String>>().join("")
  } else {
    v.to_string()
  }
}}

fn register_helpers(reg: &mut Handlebars) {
    reg.register_helper("vsuffix", Box::new(vsuffix));
}

fn generate_schemas() -> Result<(), Error> {
    let versions = ureq::get(
        "https://raw.githubusercontent.com/polywrap/wrap/master/manifest/wrap.info/versions.json",
    )
    .call()?
    .into_json::<Vec<String>>()?;

    for version in versions {
        let mut schema = ureq::get(&format!(
            "https://raw.githubusercontent.com/polywrap/wrap/master/manifest/wrap.info/{version}.json"
        ))
        .call()
        .unwrap()
        .into_json::<serde_json::Value>()
        .unwrap();

        let mut jsonref = JsonRef::new();
        jsonref
            .deref_value(&mut schema)
            .map_err(|e| Error::SchemaFetch(format!("Error dereferencing Schema. {e}")))
            .unwrap();

        let schema = serde_json::to_string_pretty(&schema).unwrap();
        fs::write(
            PathBuf::from("schemas").join(format!("{version}.json")),
            schema,
        )
        .unwrap();
    }

    Ok(())
}

fn generate_file(
    registry: &mut Handlebars,
    template_path: &str,
    data: &serde_json::Value,
    file_path: &str,
) {
    let binding = PathBuf::from(template_path);
    let name = binding.file_name().unwrap().to_str().unwrap();
    registry
        .register_template_file(name, template_path)
        .unwrap();

    let rendered_content = registry.render(name, data).unwrap();
    fs::write(file_path, rendered_content).unwrap();
}

fn main() {
    generate_schemas().unwrap();

    let mut reg = Handlebars::new();
    register_helpers(&mut reg);

    let schema_paths = fs::read_dir(PathBuf::from("schemas"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();

    let mut schemas = schema_paths
        .iter()
        .map(|schema_path| {
            let version = schema_path.file_stem().unwrap().to_str().unwrap();

            Schema {
                version: version.to_string(),
            }
        })
        .collect::<Vec<Schema>>();

    schemas.sort_by(|a, b| {
        Version::parse(&b.version)
            .unwrap()
            .cmp(&Version::parse(&a.version).unwrap())
    });

    generate_file(
        &mut reg,
        "templates/versions.hbs",
        &json!({
          "versions": schemas,
          "latest_version": schemas.last().unwrap().version,
        }),
        "./src/versions.rs",
    );

    generate_file(
        &mut reg,
        "templates/get_schemas.hbs",
        &json!({
          "versions": schemas,
        }),
        "./src/get_schemas.rs",
    );

    generate_file(
        &mut reg,
        "templates/deserialize.hbs",
        &json!({
          "versions": schemas,
          "latest_version": schemas.last().unwrap().version,
        }),
        "./src/deserialize.rs",
    );
}
