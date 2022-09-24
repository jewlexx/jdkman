use std::num::ParseIntError;

use serde::{Deserialize, Serialize};
use thiserror::Error as AsError;

mod gh;

pub type BinaryAdoptiumAssets = Vec<BinaryAdoptiumAsset>;

#[derive(Debug, AsError)]
pub enum AdoptiumApiError {
    #[error("http error")]
    Reqwest(#[from] reqwest::Error),
    #[error("json error")]
    Serde(#[from] serde_json::Error),
    #[error("failed to parse version numbers")]
    Version(#[from] ParseIntError),
}

pub async fn list_versions() -> Result<Vec<u8>, AdoptiumApiError> {
    let binary_repos = gh::get_binary_repos().await?;

    let binary_versions = binary_repos
        .iter()
        // The following line removes the "-binaries" suffix from the repo name and the "temurin" prefix
        // The repo names are formatted as follows "temurin{version}-binaries"
        // if there is a better way please let me know, but this removes the need for mutability so I'm happy
        .map(|binary| binary.name["temurin".len()..][.."-binaries".len()].parse())
        .collect::<Result<Vec<u8>, ParseIntError>>()?;

    Ok(binary_versions)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryAdoptiumAsset {
    pub binary: Binary,
    pub release_link: String,
    pub release_name: String,
    pub vendor: String,
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binary {
    pub architecture: String,
    pub download_count: i64,
    pub heap_size: String,
    pub image_type: String,
    pub installer: Installer,
    pub jvm_impl: String,
    pub os: String,
    pub package: Installer,
    pub project: String,
    pub scm_ref: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Installer {
    pub checksum: String,
    pub checksum_link: String,
    pub download_count: i64,
    pub link: String,
    pub metadata_link: String,
    pub name: String,
    pub signature_link: String,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub build: i64,
    pub major: i64,
    pub minor: i64,
    pub openjdk_version: String,
    pub security: i64,
    pub semver: String,
}
