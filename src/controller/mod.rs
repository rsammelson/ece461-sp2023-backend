mod count_commits;

pub mod scores;
pub use scores::{Score, Scores};

pub mod metrics;
pub use metrics::Metrics;

use async_trait::async_trait;
use futures::future::join_all;
use std::{path::Path, str::FromStr};
use thiserror::Error;

#[async_trait]
/// The trait that defines scoring algorithms
///
/// Arguments:
///
/// * `path`: File path to the root of a locally cloned git repository
/// * `url`: Currently unused, some object to use for API requests
trait Scorer {
    async fn score<P: AsRef<Path> + Send>(&self, path: P, url: &str) -> Score;
}

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("Do not know the `{0}` metric")]
    MetricParseError(String),
}

/// Run a set of scoring metrics and collect the results
///
/// Arguments:
///
/// * `name`: The name to be used when displaying results (typically `username/repo_name`)
/// * `path`: File path to the root of a locally cloned git repository
/// * `url`: Currently unused, some object to use for API requests
/// * `to_run`: A list of `Metric`s to run on the repository.
///
/// TODO: see if having each metric open its own Repository is slower than running in sequence
/// with the same object. Alternatively, figure out how to share the Repository object
/// bewteen threads. The docs imply this is possible, but the type does not implement `Sync`.
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
