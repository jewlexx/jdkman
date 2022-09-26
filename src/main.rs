use crate::adoptiumapi::list_versions_parsed;

#[macro_use]
extern crate tracing;

mod adoptiumapi;
mod args;
mod config;
mod logger;
mod platform;

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder().user_agent("jdkman").build().unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_subscriber();
    config::init_jdkman_home().await?;

    let args = args::JdkManArgs::parse();

    let versions = list_versions_parsed().await?;

    debug!("Found versions: {:?}", versions);

    debug!("Path: {:?}", platform::get_path());

    Ok(())
}
