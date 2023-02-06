mod api;
mod log;

use log::LogLevel;
use std::error::Error;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    log::log(LogLevel::All, LogLevel::Minimal, "Starting program...");

    let urls = [
        "https://github.com/npm/registry",
        "git://github.com/jonschlinkert/even.git",
        "https://www.npmjs.com/package/react-scripts",
    ];

    let mut tasks = task::JoinSet::new();
    for url in urls {
        tasks.spawn(fetch_repo(url));
    }

    while let Some(t) = tasks.join_next().await {
        t??
    }

    Ok(())
}

async fn fetch_repo(url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let repo = api::fetch::fetch_repo(url::Url::parse(url).unwrap()).await?;
    println!("{:#?} {:#?}", repo.is_shallow(), repo.is_bare());
    println!("{:#?}", repo.path());
    Ok(())
}
