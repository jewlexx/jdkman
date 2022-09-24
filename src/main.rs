use tokio::fs::create_dir;

use crate::adoptiumapi::{get_version_name, list_versions};

#[macro_use]
extern crate tracing;

mod adoptiumapi;
mod args;
mod env;
mod logger;

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder().user_agent("jdkman").build().unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_subscriber();

    env::init_jdkman_home().await?;

    let args = args::JdkManArgs::parse();

    let versions = list_versions().await?;

    let versions_parsed = versions
        .iter()
        .map(get_version_name)
        .collect::<Vec<String>>();

    debug!("Windows Path: {:?}", env::get_windows_path());

    debug!("Found versions: {:?}", versions_parsed);

    Ok(())
}
