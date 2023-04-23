use super::*;
use crate::{api::GithubRepositoryName, input::Weights};

use std::{collections::HashMap, iter::repeat, sync::Arc};

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !(($x - $y).abs() < $d || ($y - $x).abs() < $d) {
            panic!("{} not close enough to {}", $x, $y);
        }
    };
}

mockall::mock! {
    pub Scorer {}
    #[async_trait]
    impl Scorer for Scorer {
        async fn score<Q>(
            &self,
            repo: &tokio::sync::Mutex<git2::Repository>,
            url: &Q,
        ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>>
        where
            Q: Queryable + fmt::Display + Sync + 'static;
    }
}

pub fn get_fake_repository() -> git2::Repository {
    // this is horrible and I hate it
    // however, it should never be used since the scorer trait is being mocked
    // really this is just as unsafe as normal c code that doesn't check for null pointers
    // ... which is very
    #[allow(dead_code)]
    struct FakeRepository {
        raw: usize,
    }
    unsafe { std::mem::transmute::<FakeRepository, git2::Repository>(FakeRepository { raw: 0 }) }
}

#[test]
fn score_display_format_simple() {
    use crate::{
        api::GithubRepositoryName,
        controller::{bus_factor, Metric, Scores},
    };
    let result = Scores {
        repo_identifier: GithubRepositoryName {
            owner: "user".to_string(),
            name: "project".to_string(),
        },
        net_score: 0.8,
        scores: vec![(Metric::BusFactor(bus_factor::BusFactor()), 0.3)]
            .into_iter()
            .collect(),
    };
    assert_eq!(
        format!("{result}"),
        r#"{"URL": "https://github.com/user/project", "NET_SCORE": 0.800, "BUS_FACTOR_SCORE": 0.300}"#
    );
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
    mock.expect_score::<GithubRepositoryName>()
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
    assert!(displayed.contains("FRACTION_DEPENDENCIES_SCORE"));
    assert!(displayed.contains("FRACTION_REVIEWED_SCORE"));
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
        fraction_dependencies: 0., // TODO: Get it to work with non-zero weights
        fraction_reviewed: 0.,     // TODO: Get it to work with non-zero weights
    };

    let correct_net_score = (weights.bus_factor * 1.
        + weights.correctness_factor * 2.
        + weights.ramp_up_time * 3.
        + weights.responsiveness * 4.
        + weights.license_compatibility * 5.
        + weights.fraction_dependencies * 7.
        + weights.fraction_reviewed * 8.)
        / (weights.bus_factor
            + weights.correctness_factor
            + weights.ramp_up_time
            + weights.responsiveness
            + weights.license_compatibility
            + weights.fraction_dependencies
            + weights.fraction_reviewed)
        / len as f64;

    // this will run all metrics, giving each one a score of
    // 1 / len,  2 / len, 3 / len, etc
    let mut counter = 1..=metrics.len();
    let mut mock = MockScorer::new();
    mock.expect_score::<GithubRepositoryName>()
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
