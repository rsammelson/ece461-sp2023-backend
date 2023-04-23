#[cfg(test)]
mod tests;

mod bus_factor;
mod correctness;
mod fraction_dependencies;
mod fraction_reviewed;
mod license_compatibility;
mod ramp_up_time;
mod responsiveness;

mod scores;
pub use scores::Scores;

mod metrics;
pub use metrics::{Metric, Metrics};

use crate::{
    api::{fetch::GithubRepositoryName, graphql::Queryable},
    input, log,
    log::LogLevel,
};

use async_trait::async_trait;
use core::fmt;
use futures::future::join_all;
use std::{collections::HashMap, error::Error, sync::Arc};
use tokio::sync::Mutex;

// #[cfg_attr(test, mockall::automock)]
#[async_trait]
/// The trait that defines scoring algorithms
pub trait Scorer {
    async fn score<Q, 'a, 'b, 'c>(
        &'a self,
        repo: &'b tokio::sync::Mutex<git2::Repository>,
        repo_identifier: &'c Q,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>>
    where
        Q: Queryable + fmt::Display + Sync + 'static;
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
    repo_identifier: GithubRepositoryName,
    to_run: I,
    weights: Arc<input::Weights>,
) -> Result<Scores, Box<dyn Error + Send + Sync>>
where
    I: Iterator<Item = &'a S>,
    S: Scorer + 'a,
{
    log::log(
        LogLevel::Minimal,
        &format!("Starting analysis for {repo_identifier}"),
    );

    let scores = join_all(to_run.map(|metric| metric.score(repo, &repo_identifier)))
        .await
        .into_iter()
        .collect::<Result<HashMap<metrics::Metric, f64>, Box<dyn Error + Send + Sync>>>()?;

    Ok(calculate_net_score(
        Scores {
            scores,
            repo_identifier,
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
                Metric::FractionDependencies(_) => weights.fraction_dependencies,
                Metric::FractionReviewed(_) => weights.fraction_reviewed,
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
