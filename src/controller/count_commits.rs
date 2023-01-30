use crate::controller::*;

pub struct CountCommits();

#[async_trait]
impl Scorer for CountCommits {
    async fn score<P: AsRef<Path> + Send>(&self, path: P, _url: &str) -> Score {
        let repo = match git2::Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open repository at `{e}`"),
        };

        let mut walk = repo.revwalk().unwrap();
        walk.push_head().unwrap();
        Score {
            metric: "CountCommits".to_string(),
            score: walk.into_iter().count() as f64,
        }
    }
}

pub struct CountCommits2();

#[async_trait]
impl Scorer for CountCommits2 {
    async fn score<P: AsRef<Path> + Send>(&self, path: P, _url: &str) -> Score {
        let repo = match git2::Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open repository at `{e}`"),
        };

        let mut walk = repo.revwalk().unwrap();
        walk.push_head().unwrap();
        Score {
            metric: "CountCommits2".to_string(),
            score: walk.into_iter().count() as f64,
        }
    }
}
