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
use polywrap_core::{error::Error, invoke::{Invoker, InvokeArgs}, uri::Uri};

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
pub struct FileSystem_Module_ArgsReadFile {
    pub path: String,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystem_Module_ArgsReadFileAsString {
    pub path: String,
    pub encoding: Option<FileSystemEncoding>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystem_Module_ArgsExists {
    pub path: String,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystem_Module_ArgsWriteFile {
    pub path: String,
    pub data: Vec<u8>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystem_Module_ArgsMkdir {
    pub path: String,
    pub recursive: Option<bool>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystem_Module_ArgsRm {
    pub path: String,
    pub recursive: Option<bool>,
    pub force: Option<bool>,
}

// URI: "ens/fs.polywrap.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystem_Module_ArgsRmdir {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemModule {}

impl FileSystemModule {
    pub const URI: &'static str = "ens/fs.polywrap.eth";

    pub fn new() -> FileSystemModule {
        FileSystemModule {}
    }

    pub async fn read_file(args: &FileSystem_Module_ArgsReadFile, invoker: Arc<dyn Invoker>) -> Result<Vec<u8>, String> {
        let uri = FileSystemModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "readFile",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap())
    }

    pub async fn read_file_as_string(args: &FileSystem_Module_ArgsReadFileAsString, invoker: Arc<dyn Invoker>) -> Result<String, String> {
        let uri = FileSystemModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "readFileAsString",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap())
    }

    pub async fn exists(args: &FileSystem_Module_ArgsExists, invoker: Arc<dyn Invoker>) -> Result<bool, String> {
        let uri = FileSystemModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "exists",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap())
    }

    pub async fn write_file(args: &FileSystem_Module_ArgsWriteFile, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, String> {
        let uri = FileSystemModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "writeFile",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(Some(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap()))
    }

    pub async fn mkdir(args: &FileSystem_Module_ArgsMkdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, String> {
        let uri = FileSystemModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "mkdir",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(Some(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap()))
    }

    pub async fn rm(args: &FileSystem_Module_ArgsRm, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, String> {
        let uri = FileSystemModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "rm",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(Some(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap()))
    }

    pub async fn rmdir(args: &FileSystem_Module_ArgsRmdir, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, String> {
        let uri = FileSystemModule::URI;
        let serialized_args = InvokeArgs::UIntArray(serialize(args).unwrap());
        let args = Some(&serialized_args);
        let result = invoker.invoke(
            &Uri::try_from(uri).unwrap(),
            "rmdir",
            args,
            None,
            None
        ).await.map_err(|e| e.to_string())?;

        Ok(Some(decode(result.as_slice())
            .map_err(|e| Error::InvokeError(format!("Failed to decode result: {}", e))).unwrap()))
    }
}
// Imported Modules END //
