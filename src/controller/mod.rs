mod count_commits;

pub mod metrics;
pub use metrics::Metrics;

use async_trait::async_trait;
use futures::future::join_all;
use std::{path::Path, str::FromStr};
use thiserror::Error;

#[async_trait]
trait Scorer {
    async fn score<P: AsRef<Path> + Send>(&self, path: P, url: &str) -> Score;
}

#[derive(Debug)]
pub struct Scores {
    pub name: String,
    pub scores: Vec<Score>,
}

#[derive(Debug)]
pub struct Score {
    pub metric: String,
    pub score: f64,
}

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("Do not know the `{0}` metric")]
    MetricParseError(String),
}

pub async fn run_metrics<P: AsRef<Path> + Sync>(
    name: &str,
    path: P,
    url: &str,
    to_run: Metrics,
) -> Scores {
    Scores {
        name: name.to_string(),
        scores: join_all(to_run.iter().map(|metric| metric.score(&path, url))).await,
    }
}
