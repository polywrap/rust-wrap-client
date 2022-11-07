use std::{fs, io, path::PathBuf};

use handlebars::{handlebars_helper, Handlebars};
use polywrap_jsonref::JsonRef;
use semver::Version;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Schema {
    format: String,
}

#[derive(Serialize)]
struct FormatsData {
    formats: Vec<Schema>,
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

handlebars_helper! {fsuffix: |v: str| {
  if v.contains(".") {
    v.to_string().split(".").map(|v| v.to_string()).collect::<Vec<String>>().join("")
  } else if v.contains("_") {
    v.to_string().split("_").map(|v| v.to_string()).collect::<Vec<String>>().join("")
  } else {
    v.to_string()
  }
}}

fn register_helpers(reg: &mut Handlebars) {
    reg.register_helper("fsuffix", Box::new(fsuffix));
}

fn generate_schemas() -> Result<(), Error> {
    let versions = ureq::get(
        "https://raw.githubusercontent.com/polywrap/wrap/master/manifest/wrap.info/versions.json",
    )
    .call()?
    .into_json::<Vec<String>>()?;

    for version in versions {
        let mut schema = ureq::get(&format!(
            "https://raw.githubusercontent.com/polywrap/wrap/master/manifest/wrap.info/{}.json",
            version
        ))
        .call()
        .unwrap()
        .into_json::<serde_json::Value>()
        .unwrap();

        let mut jsonref = JsonRef::new();
        jsonref
            .deref_value(&mut schema)
            .map_err(|e| {
                Error::SchemaFetch(format!("Error dereferencing Schema. {}", e.to_string()))
            })
            .unwrap();

        let schema = serde_json::to_string_pretty(&schema).unwrap();
        fs::write(
            PathBuf::from("schemas").join(format!("{}.json", version)),
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
            let format = schema_path.file_stem().unwrap().to_str().unwrap();

            Schema {
                format: format.to_string(),
            }
        })
        .collect::<Vec<Schema>>();

    schemas.sort_by(|a, b| {
        Version::parse(&b.format)
            .unwrap()
            .cmp(&Version::parse(&a.format).unwrap())
    });

    generate_file(
        &mut reg,
        "templates/formats.hbs",
        &json!({
          "formats": schemas,
          "latest_format": schemas.last().unwrap().format,
        }),
        "./src/formats.rs",
    );

    generate_file(
        &mut reg,
        "templates/get_schemas.hbs",
        &json!({
          "formats": schemas,
        }),
        "./src/get_schemas.rs",
    );

    generate_file(
        &mut reg,
        "templates/deserialize.hbs",
        &json!({
          "formats": schemas,
          "latest_format": schemas.last().unwrap().format,
        }),
        "./src/deserialize.rs",
    );
}
