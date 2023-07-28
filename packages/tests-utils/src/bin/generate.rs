use reqwest::Client;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use zip::read::ZipArchive;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Failed to fetch file from {0}")]
    FailedToFetchFile(String),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    FileError(#[from] std::io::Error),
}
#[derive(Error, Debug)]
pub enum UnzipError {
    #[error("Failed to unzip file")]
    FailedToUnzipFile,
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error(transparent)]
    FileError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    RequestError(#[from] FetchError),
    #[error(transparent)]
    UnzipError(#[from] UnzipError),
}

async fn fetch_from_github(
    client: &Client,
    url: &str,
    source_folder: PathBuf,
) -> Result<(), FetchError> {
    let response = client.get(url).send().await?;
    fs::create_dir_all(source_folder.join("tmp").as_path())?;
    if response.status().is_success() {
        let file_content = response.bytes().await?.to_vec();
        fs::write(source_folder.join("tmp").join("cases.zip"), file_content)?;
        Ok(())
    } else {
        Err(FetchError::FailedToFetchFile(format!(
            "Failed to fetch file from {}",
            url
        )))
    }
}

fn unzip_folder(source: &Path) -> Result<(), UnzipError> {
    if fs::metadata(source.join("cases")).is_ok() {
        fs::remove_dir_all(source.join("cases"))?;
    }

    let file = fs::File::open(source.join("tmp").join("cases.zip"))?;
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = source.join("cases").join(file.mangled_name());
        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut out_file = fs::File::create(&out_path)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }
    fs::remove_dir_all(source.join("tmp"))?;
    Ok(())
}

pub async fn fetch_wrappers() -> Result<(), ExecutionError> {
    let client = Client::new();
    let tag = "0.2.0";
    let repo_name = "wrap-test-harness";
    let url = format!(
        "https://github.com/polywrap/{}/releases/download/{}/wrappers",
        repo_name, tag
    );

    let source_folder = PathBuf::from("./packages/tests-utils");
    let path_source = source_folder.as_path();

    fetch_from_github(&client, &url, path_source.to_path_buf()).await?;
    unzip_folder(path_source)?;

    println!("Wrappers folder fetch successful");
    Ok(())
}

#[tokio::main]
pub async fn main() {
    let exec = fetch_wrappers().await;
    if exec.is_err() {
        println!("Error: {:?}", exec.err());
    }
}
