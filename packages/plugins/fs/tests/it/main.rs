use polywrap_client::client::PolywrapClient;
use polywrap_core::{client::ClientConfig, uri::Uri};
use polywrap_fs_plugin::FileSystemPlugin;
use polywrap_msgpack::encode;
use polywrap_resolvers::static_resolver::{StaticResolver, StaticResolverLike};

use polywrap_plugin::package::PluginPackage;
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::path::Path;
use std::sync::Arc;
use std::{env, fs};

fn clean_up_temp_files() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();

    let temp_file_path = current_dir.join("tests/samples/tempfile.dat");
    let temp_dir_path = current_dir.join("tests/samples/tempdir");

    let temp_file_path = Path::new(&temp_file_path);
    let temp_dir_path = Path::new(&temp_dir_path);

    if temp_file_path.exists() {
        fs::remove_file(temp_file_path)?;
    }

    if temp_dir_path.exists() {
        fs::remove_dir_all(temp_dir_path)?;
    }

    Ok(())
}

fn get_client() -> PolywrapClient {
    let fs_plugin = FileSystemPlugin {};
    let plugin_pkg: PluginPackage = fs_plugin.into();
    let package = Arc::new(plugin_pkg);

    let resolver = StaticResolver::from(vec![StaticResolverLike::Package(
        Uri::try_from("plugin/file-system").unwrap(),
        package,
    )]);

    PolywrapClient::new(ClientConfig {
        resolver: Arc::new(resolver),
        interfaces: None,
        envs: None,
    })
}

#[derive(Serialize)]
struct ReadFileArgs {
    path: String,
}

#[test]
fn can_read_a_file() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let sample_file_path = current_dir.join("tests/samples/sample.txt");

    let expected_contents = fs::read(&sample_file_path).unwrap();
    let result = client.invoke::<ByteBuf>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "readFile",
        Some(
            &encode(&ReadFileArgs {
                path: sample_file_path.to_str().unwrap().to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    match result {
        Ok(contents) => assert_eq!(contents.to_vec(), expected_contents),
        Err(e) => panic!("Test failed: {:?}", e),
    }
}

#[test]
fn should_fail_reading_a_nonexistent_file() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let non_existent_file_path = current_dir.join("nonexistent.txt");

    let result = client.invoke::<Vec<u8>>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "readFile",
        Some(
            &encode(&ReadFileArgs {
                path: non_existent_file_path.to_str().unwrap().to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );
    assert!(result.is_err());
}

#[derive(Serialize)]
struct ReadFileArgsAsString {
    path: String,
    encoding: u8,
}

#[test]
fn should_read_a_utf8_encoded_file_as_a_string() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let sample_file_path = current_dir.join("tests/samples/sample.txt");

    let expected_contents = fs::read_to_string(&sample_file_path).unwrap();
    let result = client.invoke::<String>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "readFileAsString",
        Some(
            &encode(&ReadFileArgsAsString {
                path: sample_file_path.to_str().unwrap().to_string(),
                encoding: 2,
            })
            .unwrap(),
        ),
        None,
        None,
    );
    match result {
        Ok(contents) => assert_eq!(contents, expected_contents),
        Err(e) => panic!("Test failed: {:?}", e),
    }
}

// #[test]
// fn should_read_a_file_using_supported_encodings_as_a_string() {
//     let supported_encodings = vec![
//         "ascii", "utf-8", "utf-16le", "ucs-2",
//         "base64", "base64url", "latin-1", "binary", "hex"
//     ];

//     let current_dir = env::current_dir().unwrap();
//     let sample_file_path = current_dir.join("samples/sample.txt");

//     for encoding in supported_encodings {
//         let result = FileSystem_Module::read_file_as_string(&sample_file_path, encoding);

//         let expected_contents = fs::read_to_string(
//             &sample_file_path,
//             file_system_encoding_to_buffer_encoding(encoding),
//         );

//         match result {
//             Ok(contents) => assert_eq!(contents, expected_contents),
//             Err(e) => panic!("Test failed: {:?}", e),
//         }
//     }
// }

#[test]
fn should_return_whether_a_file_exists_or_not() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let sample_file_path = current_dir.join("tests/samples/sample.txt");

    let result_file_exists = client.invoke::<bool>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "exists",
        Some(
            &encode(&ReadFileArgs {
                path: sample_file_path.to_str().unwrap().to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    match result_file_exists {
        Ok(file_exists) => assert!(file_exists),
        Err(e) => panic!("Test failed: {:?}", e),
    }

    let nonexistent_file_path = current_dir.join("samples/this-file-should-not-exist.txt");

    let result_file_missing = client.invoke::<bool>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "exists",
        Some(
            &encode(&ReadFileArgs {
                path: nonexistent_file_path.to_str().unwrap().to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );
    match result_file_missing {
        Ok(file_exists) => assert!(!file_exists),
        Err(e) => panic!("Test failed: {:?}", e),
    }
}

#[derive(Serialize)]
struct WriteFileArgs {
    path: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

#[test]
fn should_write_byte_data_to_a_file() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let temp_file_path = current_dir.join("tests/samples/tempfile.dat");

    clean_up_temp_files().unwrap();

    let bytes = vec![0, 1, 2, 3];
    let result = client.invoke::<Option<bool>>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "writeFile",
        Some(
            &encode(&WriteFileArgs {
                path: temp_file_path.to_str().unwrap().to_string(),
                data: bytes,
            })
            .unwrap(),
        ),
        None,
        None,
    );
    if let Err(e) = result {
        panic!("Test failed: {:?}", e);
    }

    let expected_file_contents = fs::read(&temp_file_path).unwrap();

    assert_eq!(expected_file_contents, vec![0, 1, 2, 3]);
}

#[derive(Serialize)]
struct RmArgs {
    path: String,
    recursive: bool,
}

#[test]
fn should_remove_a_file() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let temp_file_path = current_dir.join("tests/samples/tempfile.dat");
    clean_up_temp_files().unwrap();

    fs::write(&temp_file_path, "test file contents").unwrap();

    let result = client.invoke::<bool>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "rm",
        Some(
            &encode(&RmArgs {
                path: temp_file_path.to_str().unwrap().to_string(),
                recursive: false,
            })
            .unwrap(),
        ),
        None,
        None,
    );
    if let Err(e) = result {
        panic!("Test failed: {:?}", e);
    }

    let file_exists = temp_file_path.exists();
    assert!(!file_exists);
}

#[test]
fn should_remove_a_directory_with_files_recursively() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let temp_dir_path = current_dir.join("tests/samples/tempdir");
    let file_in_dir_path = temp_dir_path.join("inner.txt");
    clean_up_temp_files().unwrap();

    fs::create_dir(&temp_dir_path).unwrap();
    fs::write(&file_in_dir_path, "test file contents").unwrap();

    let result = client.invoke::<bool>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "rm",
        Some(
            &encode(&RmArgs {
                path: temp_dir_path.to_str().unwrap().to_string(),
                recursive: true,
            })
            .unwrap(),
        ),
        None,
        None,
    );
    if let Err(e) = result {
        panic!("Test failed: {:?}", e);
    }

    let file_exists = file_in_dir_path.exists();
    assert_eq!(file_exists, false);
}

#[derive(Serialize)]
struct MkDirArgs {
    path: String,
    recursive: bool,
}

#[test]
fn should_create_a_directory() {
    let client = get_client();
    let current_dir = env::current_dir().unwrap();
    let temp_dir_path = current_dir.join("tests/samples/tempdir");
    clean_up_temp_files().unwrap();

    let result = client.invoke::<bool>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "mkdir",
        Some(
            &encode(&MkDirArgs {
                path: temp_dir_path.to_str().unwrap().to_string(),
                recursive: false,
            })
            .unwrap(),
        ),
        None,
        None,
    );
    if let Err(e) = result {
        panic!("Test failed: {:?}", e);
    }

    let directory_exists = temp_dir_path.exists();
    assert_eq!(directory_exists, true);
}

#[test]
fn should_create_a_directory_recursively() {
    let client = get_client();
    clean_up_temp_files().unwrap();

    let current_dir = env::current_dir().unwrap();
    let temp_dir_path = current_dir.join("tests/samples/tempdir");
    let dir_in_dir_path = temp_dir_path.join("inner");

    let result = client.invoke::<bool>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "mkdir",
        Some(
            &encode(&MkDirArgs {
                path: dir_in_dir_path.to_str().unwrap().to_string(),
                recursive: true,
            })
            .unwrap(),
        ),
        None,
        None,
    );
    if let Err(e) = result {
        panic!("Test failed: {:?}", e);
    }

    let directory_exists = dir_in_dir_path.exists();
    assert!(directory_exists);
}

#[derive(Serialize)]
struct RmDirArgs {
    path: String,
}

#[test]
fn should_remove_a_directory() {
    let client = get_client();
    clean_up_temp_files().unwrap();

    let current_dir = env::current_dir().unwrap();
    let temp_dir_path = current_dir.join("tests/samples/tempdir");

    fs::create_dir(&temp_dir_path).unwrap();

    let result = client.invoke::<bool>(
        &Uri::try_from("plugin/file-system").unwrap(),
        "rmdir",
        Some(
            &encode(&RmDirArgs {
                path: temp_dir_path.to_str().unwrap().to_string(),
            })
            .unwrap(),
        ),
        None,
        None,
    );
    if let Err(e) = result {
        panic!("Test failed: {:?}", e);
    }

    let directory_exists = temp_dir_path.exists();
    assert!(!directory_exists);
}