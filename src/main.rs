mod api;
mod log;

use log::LogLevel;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::log(LogLevel::All, LogLevel::Minimal, "Starting program...");

    api::fetch::fetch_repo(url::Url::parse("https://github.com/npm/registry")?).await?;
    api::fetch::fetch_repo(url::Url::parse("git://github.com/jonschlinkert/even.git")?).await?;
    api::fetch::fetch_repo(url::Url::parse(
        "https://www.npmjs.com/package/react-scripts",
    )?)
    .await?;

    Ok(())
}
