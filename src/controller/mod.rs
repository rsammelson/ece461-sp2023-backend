mod count_commits;

mod scores;
use scores::{Score, Scores};

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
