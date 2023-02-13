mod api;
mod controller;
mod input;
mod log;
mod output;

use controller::Metrics;
use input::Weights;
use log::LogLevel;

use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{sync::Mutex, task};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Could not get a URL from `{0}` because `{1}`")]
    UrlParseError(String, url::ParseError),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::log(LogLevel::Minimal, "Starting program...");
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let (weights, urls, test_mode) = input::cli::get_inputs()?;

    let metrics = Arc::new(Metrics::all());
    let weights = Arc::new(weights);

    let mut tasks = task::JoinSet::new();
    match test_mode {
        input::cli::TestMode::Normal => {
            for url in urls {
                tasks.spawn(fetch_repo_run_scores(
                    url?,
                    Arc::clone(&metrics),
                    Arc::clone(&weights),
                ));
            }
        }
        // for testing the input functionality
        // just echo back the functionality received
        input::cli::TestMode::Test => {
            println!("{weights}");
            for url in urls {
                match url {
                    Ok(url) => println!("{url}"),
                    Err(e) => println!("{e}"),
                }
            }
            return Ok(());
        }
    }

    output::console_output_sorted::print(tasks).await;

    log::log(
        LogLevel::Minimal,
        &format!("Done with run started at {start_time}"),
    );

    Ok(())
}

async fn fetch_repo_run_scores(
    url: String,
    metrics: Arc<Metrics>,
    weights: Arc<Weights>,
) -> Result<controller::Scores, Box<dyn std::error::Error + Send + Sync>> {
    let (repo_local, repo_name) = api::fetch::fetch_repo(
        url::Url::parse(&url).map_err(|err| Error::UrlParseError(url.to_owned(), err))?,
    )
    .await?;

    controller::run_metrics(&Mutex::new(repo_local), repo_name, metrics.iter(), weights).await
}
