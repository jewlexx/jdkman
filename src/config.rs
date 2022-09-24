pub struct JdkConfig {
    pub current_version: u8,
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
    }

    Ok(())
}
