use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to serialize config")]
    SerializeError(#[from] toml::ser::Error),
    #[error("failed to interact with system I/O")]
    IoError(#[from] tokio::io::Error),
}

pub static CONFIG: Mutex<JdkConfig> = Mutex::new(JdkConfig::new());

#[derive(Debug, Deserialize, Serialize)]
pub struct JdkConfig {
    pub current_version: Option<u8>,
    pub installed_versions: Vec<u8>,
}

impl JdkConfig {
    pub const fn new() -> Self {
        Self {
            current_version: None,
            installed_versions: Vec::new(),
        }
    }
}

#[instrument]
pub async fn init_jdkman_home() -> Result<(), ConfigError> {
    debug!("Creating jdkman home");

    let jdkman_path = crate::JDKMAN_HOME.to_owned();

    if !jdkman_path.exists() {
        tokio::fs::create_dir(&jdkman_path).await?;
    } else {
        debug!("jdkman home already exists");
    }

    let jdkman_config = jdkman_path.join("config.toml");

    if !jdkman_config.exists() {
        let config_bytes = toml::to_vec(&*CONFIG.lock())?;

        tokio::fs::write(&jdkman_config, config_bytes).await?;
    } else {
        debug!("jdkman config already exists");
        let config = tokio::fs::read_to_string(&jdkman_config).await?;

        let config: JdkConfig = toml::from_str(&config).unwrap();

        *CONFIG.lock() = config;
    }

    Ok(())
}
