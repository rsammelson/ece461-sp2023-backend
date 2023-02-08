mod bus_factor;
mod correctness;
mod license_compatibility;
mod ramp_up_time;
mod responsiveness;

mod scores;
pub use scores::Scores;

mod metrics;
pub use metrics::Metric;
pub use metrics::Metrics;

use crate::{api::fetch::GithubRepositoryName, input, log, log::LogLevel};

use async_trait::async_trait;
use futures::future::join_all;
use std::{collections::HashMap, error::Error, sync::Arc};
use tokio::sync::Mutex;

#[async_trait]
/// The trait that defines scoring algorithms
trait Scorer {
    async fn score(
        &self,
        repo: &Mutex<git2::Repository>,
        url: &GithubRepositoryName,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>;
}

/// Run a set of scoring metrics and collect the results
///
/// Arguments:
///
/// * `path`: File path to the root of a locally cloned git repository
/// * `url`: Some object to use for API requests. Also used as the "name" of the project
/// * `to_run`: A list of `Metric`s to run on the repository.
/// * `log_level`: Passed to each metric to use for logging
///
/// TODO: see if having each metric open its own Repository is slower than running in sequence
/// with the same object. Alternatively, figure out how to share the Repository object
/// bewteen threads. The docs imply this is possible, but the type does not implement `Sync`.
pub async fn run_metrics(
    repo: &Mutex<git2::Repository>,
    url: &GithubRepositoryName,
    to_run: Arc<Metrics>,
    weights: Arc<input::Weights>,
) -> Result<Scores, Box<dyn Error + Send + Sync>> {
    log::log(LogLevel::Minimal, &format!("Starting analysis for {url}"));

    Ok(calculate_net_scores(
        Scores {
            url: url.to_string(),
            scores: join_all(to_run.iter().map(|metric| metric.score(repo, url)))
                .await
                .into_iter()
                .zip(to_run.iter())
                .map(|(score, metric)| Ok((*metric, score?)))
                .collect::<Result<HashMap<metrics::Metric, f64>, Box<dyn Error + Send + Sync>>>()?,
            ..Scores::default()
        },
        weights,
    ))
}

fn calculate_net_scores(scores: Scores, weights: Arc<input::Weights>) -> Scores {
    let mut sum = 0.;
    let mut weight_sum = 0.;
    for (metric, score) in scores.scores.iter() {
        let weight = match metric {
            Metric::BusFactor(_) => weights.bus_factor,
            Metric::Correctness(_) => weights.correctness_factor,
            Metric::RampUpTime(_) => weights.ramp_up_time,
            Metric::Responsiveness(_) => weights.responsiveness,
            Metric::LicenseCompatibility(_) => weights.license_compatibility,
        };

        sum += score * weight;
        weight_sum += weight;
    }

    if weight_sum > 0. {
        sum /= weight_sum;
    }

    Scores {
        net_score: sum,
        ..scores
    }
}
