use crate::{ImportAbis, PolywrapManifest010, PolywrapManifest020};

pub fn migrate(manifest: PolywrapManifest010) -> PolywrapManifest020 {
    let should_have_extensions =
        manifest.build.is_some() || manifest.deploy.is_some() || manifest.meta.is_some();

    return PolywrapManifest020 {
        format: "0.2.0".to_string(),
        project: crate::PolywrapManifest020Project {
            name: manifest.name,
            type_: manifest.language,
        },
        source: crate::PolywrapManifest020Source {
            import_abis: match manifest.import_redirects {
                Some(redirects) => Some(
                    redirects
                        .into_iter()
                        .map(|redirect| ImportAbis {
                            uri: redirect.uri,
                            abi: redirect.schema,
                        })
                        .collect::<Vec<ImportAbis>>(),
                ),
                None => None,
            },
            module: manifest.module,
            schema: manifest.schema,
        },
        extensions: if should_have_extensions {
            Some(crate::PolywrapManifest020Extensions {
                build: manifest.build,
                deploy: manifest.deploy,
                meta: manifest.meta,
                infra: None,
            })
        } else {
            None
        },
    };
}
