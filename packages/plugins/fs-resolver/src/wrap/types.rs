#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use serde::{Serialize, Deserialize};
use num_bigint::BigInt;
use bigdecimal::BigDecimal as BigNumber;
use serde_json as JSON;
use std::collections::BTreeMap as Map;
use std::sync::Arc;
use polywrap_msgpack::{decode, serialize};
use polywrap_core::{invoke::{Invoker}, uri::Uri};
use polywrap_plugin::error::PluginError;

// Env START //

// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaybeUriOrManifest {
    pub uri: Option<String>,
    pub manifest: Option<Vec<u8>>,
}
// Objects END //

// Enums START //

// Enums END //

// Imported objects START //

// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum FileSystemEncoding {
    ASCII,
    UTF8,
    UTF16LE,
    UCS2,
    BASE64,
    BASE64URL,
    LATIN1,
    BINARY,
    HEX,
    _MAX_
}
// Imported enums END //

// Imported Modules START //

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModuleArgsReadFile {
    pub path: String,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModuleArgsReadFileAsString {
    pub path: String,
    pub encoding: Option<FileSystemEncoding>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModuleArgsExists {
    pub path: String,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModuleArgsWriteFile {
    pub path: String,
    pub data: Vec<u8>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModuleArgsMkdir {
    pub path: String,
    pub recursive: Option<bool>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModuleArgsRm {
    pub path: String,
    pub recursive: Option<bool>,
    pub force: Option<bool>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModuleArgsRmdir {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModule {}

impl FileSystemModule {
    pub const URI: &'static str = "ens/fs.polywrap.eth";

    pub fn new() -> FileSystemModule {
        FileSystemModule {}
    }

    pub fn read_file(args: &FileSystemModuleArgsReadFile, invoker: Arc<dyn Invoker>) -> Result<Vec<u8>, PluginError> {
        let uri = FileSystemModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "readFile",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "readFile".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(decode(result.as_slice())?)
    }

    pub fn read_file_as_string(args: &FileSystemModuleArgsReadFileAsString, invoker: Arc<dyn Invoker>) -> Result<String, PluginError> {
        let uri = FileSystemModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "readFileAsString",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "readFileAsString".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(decode(result.as_slice())?)
    }

    pub fn exists(args: &FileSystemModuleArgsExists, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError> {
        let uri = FileSystemModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "exists",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "exists".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(decode(result.as_slice())?)
    }

    pub fn write_file(args: &FileSystemModuleArgsWriteFile, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError> {
        let uri = FileSystemModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "writeFile",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "writeFile".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(Some(decode(result.as_slice())?))
    }

    pub fn mkdir(args: &FileSystemModuleArgsMkdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError> {
        let uri = FileSystemModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "mkdir",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "mkdir".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(Some(decode(result.as_slice())?))
    }

    pub fn rm(args: &FileSystemModuleArgsRm, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError> {
        let uri = FileSystemModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "rm",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "rm".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(Some(decode(result.as_slice())?))
    }

    pub fn rmdir(args: &FileSystemModuleArgsRmdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError> {
        let uri = FileSystemModule::URI;
        let serialized_args = serialize(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "rmdir",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "rmdir".to_string(),
            args: serde_json::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(Some(decode(result.as_slice())?))
    }
}
// Imported Modules END //
