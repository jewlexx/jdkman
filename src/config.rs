use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

pub static Config: Mutex<JdkConfig> = Mutex::new(JdkConfig::default());

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JdkConfig {
    pub current_version: Option<u8>,
    pub installed_versions: Vec<u8>,
}

#[instrument]
pub async fn init_jdkman_home() -> Result<(), tokio::io::Error> {
    debug!("Creating jdkman home");

    let jdkman_path = dirs::home_dir()
        .expect("failed to get home directory")
        .join(".jdkman");

    if !jdkman_path.exists() {
        tokio::fs::create_dir(&jdkman_path).await?;
    } else {
        debug!("jdkman home already exists");
    }

    let jdkman_config = jdkman_path.join("config.toml");

    if !jdkman_config.exists() {
        tokio::fs::write(&jdkman_config, "").await?;
    } else {
        debug!("jdkman config already exists");
        let config = tokio::fs::read_to_string(&jdkman_config).await?;

        let config: JdkConfig = toml::from_str(&config).unwrap();

        *Config.lock() = config;
    }

    Ok(())
}
