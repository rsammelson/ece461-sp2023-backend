use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Responsiveness();

#[async_trait]
impl Scorer for Responsiveness {
    async fn score<P: AsRef<Path> + Send>(
        &self,
        _path: P,
        url: &GithubRepositoryName,
    ) -> Result<f64, Box<dyn Error + Send + Sync>> {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze Responsiveness for {url}"),
        );

        Ok(0.)
    }
}
