use crate::controller::*;

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

const YEAR_SECS: u64 = 60 * 60 * 24 * 365;

pub struct BusFactor();

#[async_trait]
impl Scorer for BusFactor {
    async fn score<P: AsRef<Path> + Send>(
        &self,
        path: P,
        url: &str,
        log_level: LogLevel,
    ) -> Result<Score, Box<dyn Error + Send + Sync>> {
        log(
            log_level,
            LogLevel::All,
            &format!("Starting to analyze BusFactor for {url}"),
        );

        let repo = git2::Repository::open(path)?;

        let mut walk = repo.revwalk().unwrap();
        walk.set_sorting(git2::Sort::TIME)?;
        walk.set_sorting(git2::Sort::TOPOLOGICAL)?;
        walk.push_head()?;

        let mut authors = HashMap::new();

        // score each commit by size (more insertions -> higher score)
        // tally up score by author of commit
        for oid in walk.flatten() {
            let commit = repo.find_commit(oid)?;

            let commit_time = commit.time().seconds() as u64;
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now - commit_time > YEAR_SECS {
                break;
            }

            let author = commit.author();
            let name = author.name().map_or("unknown", |a| a);

            let parent = commit.parents().next();
            let commit_score = score_commit_diff(&repo, parent.as_ref(), &commit);
            let author_score = authors.get(name).map_or(0., |c| *c);

            authors.insert(name.to_string(), author_score + commit_score);
        }

        let max = authors
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater))
            .unwrap_or(&1.);
        let repo_normalized_committers: f64 =
            authors.values().filter(|s| **s > 0.01).sum::<f64>() / max;

        log(
            log_level,
            LogLevel::All,
            &format!("Done analyzing BusFactor for {url}"),
        );

        Ok(Score {
            metric: "BusFactor".to_string(),
            score: score_normalized_committers(repo_normalized_committers),
        })
    }
}

fn score_normalized_committers(committers: f64) -> f64 {
    const C: f64 = 0.75;
    let score = 1. - C.powf(committers - 1.);
    if score > 0. {
        score
    } else {
        0.
    }
}

fn score_commit_diff(
    repo: &git2::Repository,
    old: Option<&git2::Commit>,
    new: &git2::Commit,
) -> f64 {
    let old = if let Some(commit) = old {
        commit.as_object().peel_to_tree().ok()
    } else {
        None
    };
    let new = new.as_object().peel_to_tree().ok();

    let diff = repo
        .diff_tree_to_tree(old.as_ref(), new.as_ref(), None)
        .unwrap();

    let stats = diff.stats().unwrap();

    score_commit(stats.insertions())
}

fn score_commit(added: usize) -> f64 {
    let added = added as f64;
    const A: f64 = 0.025;
    const B: f64 = 2.5;
    (sigmoid(A * added - B) - sigmoid(-B)) / (1. - sigmoid(-B))
}

fn sigmoid(x: f64) -> f64 {
    1. / (1. + (-x).exp())
}
