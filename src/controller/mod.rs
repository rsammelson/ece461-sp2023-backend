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

#[cfg(test)]
mod tests {
    use super::{bus_factor, calculate_net_score, Metric, Scores};
    use crate::{
        api::GithubRepositoryName, controller::correctness, controller::*, input::Weights,
    };

    use std::{collections::HashMap, iter::repeat, sync::Arc};

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    fn get_fake_repository() -> git2::Repository {
        // this is horrible and I hate it
        // however, it should never be used since the scorer trait is being mocked
        // really this is just as unsafe as normal c code that doesn't check for null pointers
        // ... which is very
        #[allow(dead_code)]
        struct FakeRepository {
            raw: usize,
        }
        unsafe {
            std::mem::transmute::<FakeRepository, git2::Repository>(FakeRepository { raw: 0 })
        }
    }

    #[tokio::test]
    async fn score_display_contains_all() {
        let repo = Mutex::new(get_fake_repository());
        let repo_name = GithubRepositoryName {
            owner: "test".to_string(),
            name: "test".to_string(),
        };

        let metrics = Metrics::all();
        let mut metrics_iter = metrics.clone().into_iter();

        let weights = Weights::new();

        // will run all metrics and give them a score of 1
        let mut mock = MockScorer::new();
        mock.expect_score()
            .times(metrics.len())
            .returning(move |_, _| Ok((metrics_iter.next().unwrap(), 1.)));

        let result = run_metrics(
            &repo,
            repo_name,
            repeat(&mock).take(metrics.len()),
            Arc::new(weights),
        )
        .await
        .unwrap();

        let displayed = format!("{result}");

        assert!(displayed.contains("URL"));
        assert!(displayed.contains("NET_SCORE"));
        assert!(displayed.contains("RAMP_UP_SCORE"));
        assert!(displayed.contains("CORRECTNESS_SCORE"));
        assert!(displayed.contains("BUS_FACTOR_SCORE"));
        assert!(displayed.contains("RESPONSIVE_MAINTAINER_SCORE"));
        assert!(displayed.contains("LICENSE_SCORE"));
    }

    #[tokio::test]
    async fn net_score_calculation_simple() {
        let repo = Mutex::new(get_fake_repository());
        let repo_name = GithubRepositoryName {
            owner: "test".to_string(),
            name: "test".to_string(),
        };

        let metrics = Metrics::all();
        let len = metrics.len();
        let mut metrics_iter = metrics.clone().into_iter();

        let weights = Weights {
            bus_factor: 1.2,
            correctness_factor: 0.7,
            ramp_up_time: 0.1,
            responsiveness: 0.,
            license_compatibility: 1.,
        };

        let correct_net_score = (weights.bus_factor * 1.
            + weights.correctness_factor * 2.
            + weights.ramp_up_time * 3.
            + weights.responsiveness * 4.
            + weights.license_compatibility * 5.)
            / (weights.bus_factor
                + weights.correctness_factor
                + weights.ramp_up_time
                + weights.responsiveness
                + weights.license_compatibility)
            / len as f64;

        // this will run all metrics, giving each one a score of
        // 1 / len,  2 / len, 3 / len, etc
        let mut counter = 1..=metrics.len();
        let mut mock = MockScorer::new();
        mock.expect_score()
            .times(metrics.len())
            .returning(move |_, _| {
                Ok((
                    metrics_iter.next().unwrap(),
                    counter.next().unwrap() as f64 / len as f64,
                ))
            });

        let result = run_metrics(&repo, repo_name, repeat(&mock).take(len), Arc::new(weights))
            .await
            .unwrap();

        assert_delta!(result.net_score, correct_net_score, std::f64::EPSILON);
    }

    #[test]
    fn net_score_div_zero() {
        // init all weights to 0
        let weights = Weights::new();
        let scores = Scores {
            scores: HashMap::from([(Metric::BusFactor(bus_factor::BusFactor()), 1.)]),
            ..Scores::default()
        };

        let Scores { net_score, .. } = calculate_net_score(scores, Arc::new(weights));

        assert_eq!(net_score, 0.);
    }

    #[test]
    fn net_score_not_greater_than_one_given_high_weight() {
        let weights = Weights {
            bus_factor: 7.3,
            ..Weights::default()
        };
        let scores = Scores {
            scores: HashMap::from([(Metric::BusFactor(bus_factor::BusFactor()), 0.2)]),
            ..Scores::default()
        };

        let Scores { net_score, .. } = calculate_net_score(scores, Arc::new(weights));

        assert_eq!(net_score, 0.2);
    }

    #[test]
    fn net_score_ignore_unimplemented() {
        // -1 as the score means we have not implemented that metric
        // should not affect the net score though
        let weights = Weights {
            bus_factor: 1.,
            correctness_factor: 1.,
            ..Weights::default()
        };
        let scores = Scores {
            scores: HashMap::from([
                (Metric::BusFactor(bus_factor::BusFactor()), -1.),
                (Metric::Correctness(correctness::Correctness()), 0.7),
            ]),
            ..Scores::default()
        };

        let Scores { net_score, .. } = calculate_net_score(scores, Arc::new(weights));

        assert_eq!(net_score, 0.7);
    }
}
