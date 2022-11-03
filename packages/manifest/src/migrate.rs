use crate::{
    migrators::{from_010_to_020, Migrator},
    utils::find_shortest_migration_path,
    AnyManifest, PolywrapManifest,
};

pub fn migrate_polywrap_manifest(manifest: AnyManifest, to: String) -> PolywrapManifest {
    let migrators: Vec<Migrator> = vec![Migrator {
        from: "0.1.0".to_string(),
        to: "0.2.0".to_string(),
        migrate: |manifest: AnyManifest| {
            let manifest_arg = match manifest {
                AnyManifest::PolywrapManifest010(manifest) => manifest,
                _ => panic!("Invalid manifest format"),
            };

            let migrated = from_010_to_020::migrate(manifest_arg);

            return AnyManifest::PolywrapManifest020(migrated);
        },
    }];
    let from = manifest.format();

    let migration_path = find_shortest_migration_path(migrators, &from, &to);

    if migration_path.is_none() {
        panic!("No migration path found from {} to {}", from, to);
    }

    let mut migrated_manifest = manifest.clone();

    for migrator in migration_path.unwrap() {
        migrated_manifest = (migrator.migrate)(migrated_manifest);
    }

    return migrated_manifest.get_latest().unwrap();
}
