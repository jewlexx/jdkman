use std::path::PathBuf;

#[macro_use]
extern crate tracing;

mod adoptiumapi;
mod args;
mod commands;
mod config;
mod logger;
mod platform;

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder().user_agent("jdkman").build().unwrap();
    pub static ref JDKMAN_HOME: PathBuf = dirs::home_dir().unwrap().join(".jdkman");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_subscriber();
    config::init_jdkman_home().await?;

    let args = args::JdkManArgs::parse();

    debug!("Path: {:?}", platform::get_path());

    Ok(())
}
