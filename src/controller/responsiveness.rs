use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Responsiveness();

#[async_trait]
impl Scorer for Responsiveness {
    async fn score(
        &self,
        _repo: &Mutex<git2::Repository>,
        url: &GithubRepositoryName,
    ) -> Result<f64, Box<dyn Error + Send + Sync>> {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze Responsiveness for {url}"),
        );

        Ok(0.)
    }
}
