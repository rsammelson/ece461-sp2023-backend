use crate::{controller::Scores, log, log::LogLevel};
use std::error::Error;
use tokio::task::JoinSet;

pub async fn print(mut tasks: JoinSet<Result<Scores, Box<dyn Error + Send + Sync>>>) {
    let mut all_scores = Vec::with_capacity(tasks.len());

    while let Some(Ok(t)) = tasks.join_next().await {
        match t {
            Ok(score) => all_scores.push(score),
            Err(e) => log::log(LogLevel::Minimal, &format!("{e}")),
        }
    }

    all_scores.sort_by(|a, b| a.net_score.partial_cmp(&b.net_score).unwrap());

    for repo in all_scores {
        println!("{repo}");
    }
}
