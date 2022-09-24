use adoptiumapi::BinaryAdoptiumAssets;

use crate::adoptiumapi::{get_version_name, list_versions};

mod adoptiumapi;
mod args;

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder().user_agent("jdkman").build().unwrap();
}

const JDK_TEST_URL: &str = "https://api.adoptium.net/v3/assets/version/17.0.4.1?architecture=x64&c_lib=musl&heap_size=normal&image_type=jdk&jvm_impl=hotspot&lts=true&os=windows&page=0&page_size=10&project=jdk&release_type=ga&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse";

const DL_URL: &str = "https://github.com/adoptium/temurin18-binaries/releases/download/jdk-18.0.2.1+1/OpenJDK18U-jdk_x64_windows_hotspot_18.0.2.1_1.zip";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::JdkManArgs::parse();

    let asset = {
        let mut resp: BinaryAdoptiumAssets = reqwest::get(JDK_TEST_URL).await?.json().await?;
        resp.remove(0)
    };

    let versions = list_versions().await?;

    let versions_parsed = versions
        .iter()
        .map(get_version_name)
        .collect::<Vec<String>>();

    println!("Hello, world!");

    Ok(())
}
