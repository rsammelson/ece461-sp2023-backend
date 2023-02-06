use crate::controller::*;

/// This is just an example of how to write a metric scoring algorithm
pub struct CountCommits();

#[async_trait]
impl Scorer for CountCommits {
    async fn score<P: AsRef<Path> + Send>(
        &self,
        path: P,
        _url: &str,
        _log_level: LogLevel,
    ) -> Result<Score, Box<dyn Error + Send + Sync>> {
        let repo = match git2::Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open repository at `{e}`"),
        };

        let mut walk = repo.revwalk().unwrap();
        walk.push_head()?;
        Ok(Score {
            metric: "CountCommits".to_string(),
            score: walk.into_iter().count() as f64,
        })
    }
}
