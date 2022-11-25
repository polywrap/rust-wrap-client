use std::{sync::Arc, path::Path};

use async_trait::async_trait;
use polywrap_core::{error::Error, invoke::Invoker};
use wrap::{
    module::{ArgsGetFile, ArgsTryResolveUri, Module},
    types::{MaybeUriOrManifest, FileSystemModule, FileSystem_Module_ArgsExists, FileSystem_Module_ArgsReadFile},
};
pub mod wrap;

pub struct FileSystemResolverPlugin {}

#[async_trait]
impl Module for FileSystemResolverPlugin {
    async fn try_resolve_uri(
        &mut self,
        args: &ArgsTryResolveUri,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<MaybeUriOrManifest>, Error> {
        if args.authority != "fs" && args.authority != "file" {
            return Ok(None);
        };
        let manifest_search_pattern = "wrap.info";

        let manifest_path = Path::new(&args.path).join(Path::new(manifest_search_pattern)).canonicalize().unwrap();
        let manifest_exists_result = FileSystemModule::exists(
          &FileSystem_Module_ArgsExists {
            path: manifest_path.to_str().unwrap().to_string()
          }, invoker.clone()).await;
          
        let manifest = if manifest_exists_result.is_ok() {
          let manifest_result = FileSystemModule::read_file(
            &FileSystem_Module_ArgsReadFile {
              path: manifest_path.to_str().unwrap().to_string()
            }, invoker).await;

            if let Ok(manifest) = manifest_result {
              Some(manifest)
            } else {
              None
            }
        } else {
          None
        };

        Ok(Some(MaybeUriOrManifest {
            uri: None,
            manifest,
        }))
    }

    async fn get_file(
        &mut self,
        args: &ArgsGetFile,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<Vec<u8>>, Error> {
        let resolve_result = FileSystemModule::read_file(
            &FileSystem_Module_ArgsReadFile { path: args.path.clone() },
            invoker,
        )
        .await;

        let file_result = match resolve_result {
            Ok(r) => Some(r),
            Err(_) => None
        };

        Ok(file_result)
    }
}

impl_traits!(FileSystemResolverPlugin);
