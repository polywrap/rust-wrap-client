use crate::wrap::wrap_info::get_manifest;
use std::{fs, path::Path, sync::Arc};

use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, implementor::plugin_impl};
use wrap::module::{
    ArgsExists, ArgsMkdir, ArgsReadFile, ArgsReadFileAsString, ArgsRm, ArgsRmdir, ArgsWriteFile,
    Module,
};
pub mod wrap;
use serde_bytes::ByteBuf;

#[derive(Debug)]
pub struct FileSystemPlugin;

#[plugin_impl]
impl Module for FileSystemPlugin {
    fn read_file(
        &mut self,
        args: &ArgsReadFile,
        _: Arc<dyn Invoker>,
    ) -> Result<ByteBuf, PluginError> {
        let result = fs::read(&args.path)
            .map_err(|e| FileSystemPluginError::ReadFileError(args.path.clone(), e))?;

        Ok(ByteBuf::from(result))
    }

    fn read_file_as_string(
        &mut self,
        args: &ArgsReadFileAsString,
        _: Arc<dyn Invoker>,
    ) -> Result<String, PluginError> {
        // TODO: Make use of args.encoding variable
        Ok(fs::read_to_string(&args.path)
            .map_err(|e| FileSystemPluginError::ReadFileAsStringError(args.path.clone(), e))?)
    }

    fn exists(&mut self, args: &ArgsExists, _: Arc<dyn Invoker>) -> Result<bool, PluginError> {
        Ok(Path::new(&args.path).exists())
    }

    fn write_file(
        &mut self,
        args: &ArgsWriteFile,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        fs::write(Path::new(&args.path), &args.data)
            .map_err(|e| FileSystemPluginError::WriteFileError(args.path.clone(), e))?;

        Ok(Some(true))
    }

    fn mkdir(
        &mut self,
        args: &ArgsMkdir,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        let recursive = if let Some(recursive) = args.recursive {
            recursive
        } else {
            false
        };

        let path = Path::new(&args.path);

        if recursive {
            fs::create_dir_all(path)
                .map_err(|e| FileSystemPluginError::MkDirRecursiveError(args.path.clone(), e))?;
        } else {
            fs::create_dir(path)
                .map_err(|e| FileSystemPluginError::MkDirError(args.path.clone(), e))?;
        }

        Ok(Some(true))
    }

    fn rm(&mut self, args: &ArgsRm, _: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError> {
        let recursive = if let Some(recursive) = args.recursive {
            recursive
        } else {
            false
        };

        let force = if let Some(force) = args.force {
            force
        } else {
            false
        };

        let path = Path::new(&args.path);

        if path.is_dir() {
            if force {
                rm_rf::ensure_removed(path)
                    .map_err(|e| FileSystemPluginError::RmRfError(args.path.clone(), e))?;
            } else if recursive {
                fs::remove_dir_all(path).map_err(|e| {
                    FileSystemPluginError::RmDirRecursiveError(args.path.clone(), e)
                })?;
            } else {
                fs::remove_dir(path)
                    .map_err(|e| FileSystemPluginError::RmDirError(args.path.clone(), e))?;
            }
        } else {
            fs::remove_file(path)
                .map_err(|e| FileSystemPluginError::RmFileError(args.path.clone(), e))?;
        }

        Ok(Some(true))
    }

    fn rmdir(
        &mut self,
        args: &ArgsRmdir,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<bool>, PluginError> {
        fs::remove_dir(&args.path)
            .map_err(|e| FileSystemPluginError::RmDirError(args.path.clone(), e))?;

        Ok(Some(true))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FileSystemPluginError {
    #[error("Error during RmRf, path: {0}, Message: `{1}`")]
    RmRfError(String, rm_rf::Error),
    #[error("Error removing directory, path: {0}, Message: `{1}`")]
    RmDirError(String, std::io::Error),
    #[error("Error recursively removing directory, path: {0}, Message: `{1}`")]
    RmDirRecursiveError(String, std::io::Error),
    #[error("Error removing file, path: {0}, Message: `{1}`")]
    RmFileError(String, std::io::Error),
    #[error("Error reading file, path: {0}, Message: `{1}`")]
    ReadFileError(String, std::io::Error),
    #[error("Error reading file as string, path: {0}, Message: `{1}`")]
    ReadFileAsStringError(String, std::io::Error),
    #[error("Error writing to file, path: {0}, Message: `{1}`")]
    WriteFileError(String, std::io::Error),
    #[error("Error creating directory, path: {0}, Message: `{1}`")]
    MkDirError(String, std::io::Error),
    #[error("Error recursively creating directory, path: {0}, Message: `{1}`")]
    MkDirRecursiveError(String, std::io::Error),
}

impl From<FileSystemPluginError> for PluginError {
    fn from(e: FileSystemPluginError) -> Self {
        PluginError::InvocationError {
            exception: e.to_string(),
        }
    }
}
