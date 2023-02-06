mod api;
mod controller;
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
        tasks.spawn(fetch_repo_run_scores(url));
    }

    while let Some(score) = tasks.join_next().await {
        println!("{}", score??);
    }

    Ok(())
}

async fn fetch_repo_run_scores(
    url: &str,
) -> Result<controller::Scores, Box<dyn Error + Send + Sync>> {
    let repo = api::fetch::fetch_repo(url::Url::parse(url).unwrap()).await?;
    let path = repo.path();

    log::log(LogLevel::All, LogLevel::All, &format!("{path:?}"));

    controller::run_metrics(path, url, &controller::Metrics::all(), LogLevel::All).await
}
