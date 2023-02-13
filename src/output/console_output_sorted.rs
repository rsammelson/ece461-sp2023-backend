use crate::{controller::Scores, log, log::LogLevel};
use std::error::Error;
use tokio::task::JoinSet;

pub async fn sort(mut tasks: JoinSet<Result<Scores, Box<dyn Error + Send + Sync>>>) -> Vec<Scores> {
    let mut all_scores = Vec::with_capacity(tasks.len());

    while let Some(Ok(t)) = tasks.join_next().await {
        match t {
            Ok(score) => all_scores.push(score),
            Err(e) => log::log(LogLevel::Minimal, &format!("{e}")),
        }
    }

    all_scores.sort_by(|a, b| b.net_score.partial_cmp(&a.net_score).unwrap());
    all_scores
}

pub async fn print(tasks: JoinSet<Result<Scores, Box<dyn Error + Send + Sync>>>) {
    for repo in sort(tasks).await {
        println!("{repo}");
    }
}
