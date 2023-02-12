use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct RampUpTime();

#[async_trait]
impl Scorer for RampUpTime {
    async fn score(
        &self,
        _repo: &Mutex<git2::Repository>,
        url: &GithubRepositoryName,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>> {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze RampUpTime for {url}"),
        );

        Ok((Metric::RampUpTime(RampUpTime()), -1.))
    }
}
