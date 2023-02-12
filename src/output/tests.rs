use super::*;
use crate::{api::GithubRepositoryName, controller::Scores};

use std::{collections::HashMap, error::Error};
use tokio::task::JoinSet;

// async closures not stable
async fn hand_back_repo(
    scores: Result<Scores, Box<dyn Error + Send + Sync>>,
) -> Result<Scores, Box<dyn Error + Send + Sync>> {
    scores
}

#[tokio::test]
async fn simple_sort() {
    let mut tasks = JoinSet::new();
    for i in 0..3 {
        tasks.spawn(hand_back_repo(Ok(Scores {
            url: GithubRepositoryName {
                owner: "test".to_string(),
                name: format!("{i}"),
            },
            net_score: i as f64,
            scores: HashMap::new(),
        })));
    }

    let result = console_output_sorted::sort(tasks).await;
    assert!(result
        .windows(2)
        .all(|pair| pair[0].net_score >= pair[1].net_score));
}
