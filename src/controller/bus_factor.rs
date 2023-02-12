use crate::controller::*;

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

const YEAR_SECS: u64 = 60 * 60 * 24 * 365;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BusFactor();

#[async_trait]
impl Scorer for BusFactor {
    async fn score(
        &self,
        repo: &Mutex<git2::Repository>,
        url: &GithubRepositoryName,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>> {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze BusFactor for {url}"),
        );

        let repo = repo.lock().await;

        let mut walk = repo.revwalk()?;
        walk.set_sorting(git2::Sort::TIME)?;
        walk.push_head()?;

        let mut authors = HashMap::new();

        // score each commit by size (more insertions -> higher score)
        // tally up score by author of commit
        for oid in walk.flatten() {
            let commit = repo.find_commit(oid)?;

            let commit_time = commit.time().seconds() as u64;
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if now - commit_time > YEAR_SECS {
                break;
            }

            let author = commit.author();
            let name = author.name().map_or("unknown", |a| a);

            let parent = commit.parents().next();
            let commit_score = score_commit_diff(&repo, parent.as_ref(), &commit)?;
            let author_score = authors.get(name).map_or(0., |c| *c);

            authors.insert(name.to_string(), author_score + commit_score);
        }

        let max = authors
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater))
            .unwrap_or(&1.);
        let repo_normalized_committers = authors.values().sum::<f64>() / max;

        log::log(
            LogLevel::All,
            &format!("Done analyzing BusFactor for {url}"),
        );

        Ok((
            Metric::BusFactor(BusFactor()),
            score_normalized_committers(repo_normalized_committers),
        ))
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
) -> Result<f64, Box<dyn Error + Sync + Send>> {
    let old = if let Some(commit) = old {
        Some(commit.as_object().peel_to_tree()?)
    } else {
        None
    };
    let new = new.as_object().peel_to_tree()?;

    let diff = repo.diff_tree_to_tree(old.as_ref(), Some(&new), None)?;

    let stats = diff.stats()?;

    Ok(score_commit(stats.insertions()))
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

#[cfg(test)]
mod tests {
    use crate::controller::bus_factor::score_commit;

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
}
