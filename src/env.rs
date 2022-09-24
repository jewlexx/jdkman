use thiserror::Error as AsError;

#[instrument]
pub async fn init_jdkman_home() -> Result<(), tokio::io::Error> {
    debug!("Creating jdkman home");

    let jdkman_path = dirs::home_dir()
        .expect("failed to get home directory")
        .join(".jdkman");

    if !jdkman_path.exists() {
        tokio::fs::create_dir(jdkman_path).await?;
    } else {
        debug!("jdkman home already exists");
    }

    Ok(())
}

#[derive(Debug, AsError)]
pub enum EnvError {
    #[error("could not get PATH environment variable")]
    PathNotFound,
    #[error("failed to pass OsString")]
    OsStringError(std::ffi::OsString),
}

#[cfg(windows)]
pub fn get_java_home() -> Option<String> {
    std::env::var_os("JAVA_HOME").map(|path| path.into_string().unwrap())
}

#[derive(Debug)]
pub struct Path(Vec<String>);

impl From<String> for Path {
    fn from(path: String) -> Self {
        let path_vec = path
            .split(';')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Path(path_vec)
    }
}

#[cfg(windows)]
pub fn get_windows_path() -> Result<Path, EnvError> {
    let path_var = match std::env::var_os("PATH") {
        Some(v) => v.into_string(),
        None => return Err(EnvError::PathNotFound),
    };

    match path_var {
        Ok(v) => Ok(Path::from(v)),
        Err(e) => Err(EnvError::OsStringError(e)),
    }
}
