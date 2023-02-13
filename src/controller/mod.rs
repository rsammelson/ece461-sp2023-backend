mod bus_factor;
mod correctness;
mod license_compatibility;
mod ramp_up_time;
mod responsiveness;

mod scores;
pub use scores::Scores;

mod metrics;
pub use metrics::{Metric, Metrics};

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
/// * `repo`: Object representing on-disk git repository
/// * `url`: Object to use for API requests.
/// * `to_run`: A list of `Metric`s to run on the repository.
/// * `log_level`: Passed to each metric to use for logging
///
/// Returns `Err()` iff any of the metric calculations return `Err()`
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
                // this silliness is because `metric.score()` is a future, but `metric` is not
                .zip(to_run.iter())
                // this just reorders the tuple while propogating errors upwards
                .map(|(score, metric)| Ok((*metric, score?)))
                .collect::<Result<HashMap<metrics::Metric, f64>, Box<dyn Error + Send + Sync>>>()?,
            ..Scores::default()
        },
        weights,
    ))
}

fn calculate_net_scores(scores: Scores, weights: Arc<input::Weights>) -> Scores {
    let (score_sum, weight_sum) = scores
        .scores
        .iter()
        .map(|(metric, score)| {
            let weight = match metric {
                Metric::BusFactor(_) => weights.bus_factor,
                Metric::Correctness(_) => weights.correctness_factor,
                Metric::RampUpTime(_) => weights.ramp_up_time,
                Metric::Responsiveness(_) => weights.responsiveness,
                Metric::LicenseCompatibility(_) => weights.license_compatibility,
            };
            (score * weight, weight)
        })
        .fold((0., 0.), |(score_sum, weight_sum), (score, weight)| {
            (score_sum + score, weight_sum + weight)
        });

    Scores {
        net_score: if weight_sum != 0. {
            score_sum / weight_sum
        } else {
            0.
        },
        ..scores
    }
}
