use crate::adoptiumapi::{get_version_name, list_versions};

#[macro_use]
extern crate tracing;

mod adoptiumapi;
mod args;
mod logger;

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder().user_agent("jdkman").build().unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_subscriber();

    let args = args::JdkManArgs::parse();

    let versions = list_versions().await?;

    let versions_parsed = versions
        .iter()
        .map(get_version_name)
        .collect::<Vec<String>>();

    debug!("Found versions: {:?}", versions_parsed);

    Ok(())
}
