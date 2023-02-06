mod api;
mod controller;
mod log;

use log::LogLevel;

use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::task;

mod output;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    log::log(LogLevel::Minimal, "Starting program...");
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let urls = [
        "https://github.com/facebook/react",
        "https://github.com/npm/registry",
        "git://github.com/jonschlinkert/even.git",
        "https://www.npmjs.com/package/react-scripts",
    ];

    let mut tasks = task::JoinSet::new();
    for url in urls {
        tasks.spawn(fetch_repo_run_scores(url));
    }

    output::console_output_sorted::print(tasks).await;

    log::log(
        LogLevel::Minimal,
        &format!("Done with run started at {start_time}"),
    );

    Ok(())
}

async fn fetch_repo_run_scores(
    url: &str,
) -> Result<controller::Scores, Box<dyn Error + Send + Sync>> {
    let (repo_local, repo_name) = api::fetch::fetch_repo(url::Url::parse(url).unwrap()).await?;
    let path = repo_local.path();

    controller::run_metrics(path, &repo_name, &controller::Metrics::all()).await
}
