use crate::{
    formats::{AnyManifest, PolywrapManifest},
    get_migrators::get_migrators,
    utils::find_shortest_migration_path,
};

pub fn migrate_polywrap_manifest(manifest: AnyManifest, to: String) -> PolywrapManifest {
    let from = manifest.format();

    let migration_path = find_shortest_migration_path(get_migrators(), &from, &to);

    if migration_path.is_none() {
        panic!("No migration path found from {} to {}", from, to);
    }

    let mut migrated_manifest = manifest.clone();

    for migrator in migration_path.unwrap() {
        migrated_manifest = (migrator.migrate)(migrated_manifest);
    }

    return migrated_manifest.get_latest().unwrap();
}
