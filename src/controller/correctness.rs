use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Correctness();

#[async_trait]
impl Scorer for Correctness {
    async fn score(
        &self,
        _repo: &Mutex<git2::Repository>,
        url: &GithubRepositoryName,
    ) -> Result<f64, Box<dyn Error + Send + Sync>> {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze Correctness for {url}"),
        );

        Ok(0.)
    }
}
