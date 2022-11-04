use std::{fs, path::PathBuf};

use handlebars::{handlebars_helper, Handlebars};
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

#[derive(Serialize, Debug)]
struct Migrator {
    from: String,
    to: String,
    name: String,
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

fn register_templates(reg: &mut Handlebars) {
    reg.register_template_file("formats", "templates/formats.hbs")
        .unwrap();
    reg.register_template_file("get_schemas", "templates/get_schemas.hbs")
        .unwrap();
    reg.register_template_file("get_migrators", "templates/get_migrators.hbs")
        .unwrap();

    reg.register_template_file("migrators_mod", "templates/migrators_mod.hbs")
        .unwrap();
}

fn extract_formats_from_migrator_name(name: &str) -> (String, String) {
    let formats = name.replace("from_", "").replace("_to_", " ");

    let mut parts = formats.split(" ");
    let from = parts.next().unwrap().to_string();
    let to = parts.next().unwrap().to_string();
    (from, to)
}

fn main() {
    let mut reg = Handlebars::new();
    register_helpers(&mut reg);
    register_templates(&mut reg);

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

    let migrator_names = fs::read_dir(PathBuf::from("src/migrators"))
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let name = path.file_stem().unwrap().to_str().unwrap();
            name.to_string()
        })
        .filter(|path| path.starts_with("from"))
        .collect::<Vec<_>>();

    let migrators = migrator_names
        .iter()
        .map(|name| {
            let (from, to) = extract_formats_from_migrator_name(name);
            Migrator {
                from,
                to,
                name: name.to_string(),
            }
        })
        .collect::<Vec<Migrator>>();

    let formats_content = reg
        .render(
            "formats",
            &json!({
              "formats": schemas,
              "latest_format": schemas.last().unwrap().format,
            }),
        )
        .unwrap();

    let get_schemas_content = reg
        .render(
            "get_schemas",
            &json!({
              "formats": schemas,
            }),
        )
        .unwrap();

    let get_migrators_content = reg
        .render(
            "get_migrators",
            &json!({
              "migrators": migrators,
            }),
        )
        .unwrap();

    let migrators_mod_content = reg
        .render(
            "migrators_mod",
            &json!({
              "migrators": migrators,
            }),
        )
        .unwrap();

    fs::write(PathBuf::from("./src/formats.rs"), formats_content).unwrap();
    fs::write(PathBuf::from("./src/get_schemas.rs"), get_schemas_content).unwrap();
    fs::write(
        PathBuf::from("./src/get_migrators.rs"),
        get_migrators_content,
    )
    .unwrap();
    fs::write(
        PathBuf::from("./src/migrators/mod.rs"),
        migrators_mod_content,
    )
    .unwrap();
}
