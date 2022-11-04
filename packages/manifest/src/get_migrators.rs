use super::{migrator::Migrator};
use super::migrators::from_0_1_0_to_0_2_0;

pub fn get_migrators() -> Vec<Migrator> {
    return vec![
        Migrator {
            from: "0.1.0".to_string(),
            to: "0.2.0".to_string(),
            migrate: |manifest: AnyManifest| {
                let manifest_arg = match manifest {
                    AnyManifest::PolywrapManifest010(manifest) => manifest,
                    _ => panic!("Invalid manifest format"),
                };

                let migrated = from_0_1_0_to_0_2_0::migrate(manifest_arg);

                return AnyManifest::PolywrapManifest020(migrated);
            },
        },
        Migrator {
            from: "0.1.0".to_string(),
            to: "0.2.0".to_string(),
            migrate: |manifest: AnyManifest| {
                let manifest_arg = match manifest {
                    AnyManifest::PolywrapManifest010(manifest) => manifest,
                    _ => panic!("Invalid manifest format"),
                };

                let migrated = from_0_1_0_to_0_2_0::migrate(manifest_arg);

                return AnyManifest::PolywrapManifest020(migrated);
            },
        },
    ];
}
