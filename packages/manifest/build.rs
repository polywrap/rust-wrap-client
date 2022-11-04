use std::{fs, path::PathBuf};

use handlebars::{handlebars_helper, Handlebars};
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

handlebars_helper! {fsuffix: |v: str| {
  v.to_string().split(".").map(|v| v.to_string()).collect::<Vec<String>>().join("")
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

    //TODO: sort by version
    schemas.sort_by(|a, b| a.format.cmp(&b.format));

    let migrator_paths = fs::read_dir(PathBuf::from("src/migrators"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.starts_with("from"))
        .collect::<Vec<_>>();

    let mut migrators = migrator_paths.iter().map(|path| {
        let from = path.file_stem().unwrap().to_str().unwrap().split("_").collect::<Vec<_>>()[1];
        let to = path.file_stem().unwrap().to_str().unwrap().split("_").collect::<Vec<_>>()[3];

        (from.to_string(), to.to_string())
    }).collect::<Vec<(String, String)>>();

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
              "formats": schemas,
            }),
        )
        .unwrap();

    fs::write(PathBuf::from("./src/formats.rs"), formats_content.clone()).unwrap();
    fs::write(
        PathBuf::from("./src/get_schemas.rs"),
        get_schemas_content,
    )
    .unwrap();
    fs::write(
        PathBuf::from("./src/get_migrators.rs"),
        get_migrators_content,
    ).unwrap();
}
