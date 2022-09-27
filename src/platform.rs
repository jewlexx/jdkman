#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("environment error")]
    VarError(#[from] std::env::VarError),
}

pub fn get_java_home() -> Result<String, EnvError> {
    Ok(std::env::var("JAVA_HOME")?)
}

#[derive(Debug)]
pub struct Path(Vec<String>);

impl From<String> for Path {
    fn from(path: String) -> Self {
        const SPLIT_CHAR: char = {
            if cfg!(windows) {
                ';'
            } else {
                ':'
            }
        };

        let path_vec = path
            .split(SPLIT_CHAR)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Path(path_vec)
    }
}

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        pub use platform_win::*;
    } else if #[cfg(unix)] {
        pub use platform_nix::*;
    }
}

#[instrument(level = "debug")]
pub fn get_path() -> Result<Path, EnvError> {
    let path_var = std::env::var("PATH")?;

    Ok(Path::from(path_var))
}
