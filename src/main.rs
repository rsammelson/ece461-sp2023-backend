mod api;
mod controller;
mod input;
mod log;
mod output;
mod cli;

use log::LogLevel;

use std::{
    error::Error,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::task;

use controller::Metrics;
use input::Weights;

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

    let metrics = Arc::new(Metrics::all());
    let weights = Arc::new(Weights::new());

    let mut tasks = task::JoinSet::new();
    for url in urls {
        tasks.spawn(fetch_repo_run_scores(
            url,
            Arc::clone(&metrics),
            Arc::clone(&weights),
        ));
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
    metrics: Arc<Metrics>,
    weights: Arc<Weights>,
) -> Result<controller::Scores, Box<dyn Error + Send + Sync>> {
    let (repo_local, repo_name) = api::fetch::fetch_repo(url::Url::parse(url).unwrap()).await?;
    let path = repo_local.path();

    controller::run_metrics(path, &repo_name, metrics, weights).await
}
