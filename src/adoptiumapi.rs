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
        .map(|binary| {
            // This block removes all non-numeric characters from the binary name
            // This will leave the version number

            let name = binary.name.to_owned();

            let int_chars = name
                .chars()
                .map(|c| if c.is_numeric() { c } else { ' ' })
                .collect::<String>();

            int_chars.parse()
        })
        .collect::<Result<Vec<u8>, ParseIntError>>()?;

    Ok(binary_versions)
}

/// This won't work if they update the binary repo names
///
/// But it works for now :)
pub fn get_version_name(version: impl std::ops::Deref<Target = u8>) -> String {
    format!("temurin{}-binaries", *version)
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
