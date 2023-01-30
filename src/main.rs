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
    println!("{:#?} {:#?}", repo.is_shallow(), repo.is_bare());
    println!("{:#?}", repo.path());

    let path = repo.path();

    Ok(controller::run_metrics(
        "name_of_repo",
        path,
        url,
        controller::Metrics::try_from(vec!["CountCommits", "CountCommits2"]).unwrap(),
    )
    .await)
}
