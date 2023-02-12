#[cfg(test)]
mod tests;

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
use mockall::automock;
use std::{collections::HashMap, error::Error, sync::Arc};
use tokio::sync::Mutex;

#[automock]
#[async_trait]
/// The trait that defines scoring algorithms
pub trait Scorer {
    async fn score<'a, 'b, 'c>(
        &'a self,
        repo: &'b tokio::sync::Mutex<git2::Repository>,
        url: &'c GithubRepositoryName,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>>;
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
pub async fn run_metrics<'a, I, S>(
    repo: &Mutex<git2::Repository>,
    url: GithubRepositoryName,
    to_run: I,
    weights: Arc<input::Weights>,
) -> Result<Scores, Box<dyn Error + Send + Sync>>
where
    I: Iterator<Item = &'a S>,
    S: Scorer + 'a,
{
    log::log(LogLevel::Minimal, &format!("Starting analysis for {url}"));

    let scores = join_all(to_run.map(|metric| metric.score(repo, &url)))
        .await
        .into_iter()
        .collect::<Result<HashMap<metrics::Metric, f64>, Box<dyn Error + Send + Sync>>>()?;

    Ok(calculate_net_score(
        Scores {
            scores,
            url,
            ..Scores::default()
        },
        weights,
    ))
}

fn calculate_net_score(scores: Scores, weights: Arc<input::Weights>) -> Scores {
    let (score_sum, weight_sum) = scores
        .scores
        .iter()
        .filter(|(_metric, score)| **score >= 0.)
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
