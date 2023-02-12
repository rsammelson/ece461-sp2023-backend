use crate::{api::GithubRepositoryName, controller::bus_factor::*, input::Weights};

use std::env::current_dir;

#[test]
fn score_commit_zero() {
    assert_eq!(score_commit(0), 0.);
}

#[test]
fn score_commit_small() {
    let score = score_commit(50);
    assert!(0.1 < score);
    assert!(score < 0.2);
}

#[test]
fn score_commit_normal() {
    let score = score_commit(100);
    assert!(0.4 < score);
    assert!(score < 0.5);
}

#[test]
fn score_commit_large() {
    let score = score_commit(300);
    assert!(0.9 < score);
    assert!(score < 1.);
}

// this is really not a great way to test this metric.
// Ideally would have multiple repos to pull from and have estimated scores for each
// However, don't want to connect to the internet for tests
#[tokio::test]
async fn multiple_commits_better() {
    let metrics = Metrics(vec![Metric::BusFactor(BusFactor())]);
    let weights = Arc::new(Weights {
        bus_factor: 1.,
        ..Weights::new()
    });

    let multiple_commits_repo =
        git2::Repository::open(current_dir().unwrap().join("tests/multiple_commits_repo")).unwrap();
    let multiple_name = GithubRepositoryName {
        owner: "test".to_string(),
        name: "multiple".to_string(),
    };

    let single_commit_repo =
        git2::Repository::open(current_dir().unwrap().join("tests/single_commit_repo")).unwrap();
    let single_name = GithubRepositoryName {
        owner: "test".to_string(),
        name: "single".to_string(),
    };

    let multiple_result = run_metrics(
        &Mutex::new(multiple_commits_repo),
        multiple_name,
        metrics.iter(),
        Arc::clone(&weights),
    )
    .await
    .unwrap();

    let single_result = run_metrics(
        &Mutex::new(single_commit_repo),
        single_name,
        metrics.iter(),
        Arc::clone(&weights),
    )
    .await
    .unwrap();

    assert!(multiple_result.net_score > single_result.net_score);
}
