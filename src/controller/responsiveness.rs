use crate::{api::graphql::Queryable, controller::*};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Responsiveness();

#[async_trait]
impl Scorer for Responsiveness {
    async fn score(
        &self,
        _repo: &Mutex<git2::Repository>,
        repo_identifier: &GithubRepositoryName,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>> {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze Responsiveness for {repo_identifier}"),
        );

        let response_times = repo_identifier.query_responsiveness().await?;
        let response_times = response_times.iter().map(|r| r.as_ref().ok()).flatten();

        let (n, total_time) = response_times.fold((0, chrono::Duration::zero()), |(n, sum), t| {
            (n + 1, sum + *t)
        });

        if n <= 0 {
            log::log(
                LogLevel::All,
                &format!("Got response time 0 for {repo_identifier}, due to no responses"),
            );

            Ok((Metric::Responsiveness(Responsiveness()), 0.))
        } else {
            let average_time = (total_time / n).num_seconds() as f64;
            let x: f64 = average_time / (60. * 60. * 24. * 3.) + 1.;
            let score = 1. / x;

            log::log(
                LogLevel::All,
                &format!("Got responsiveness {score} for {repo_identifier}"),
            );

            Ok((Metric::Responsiveness(Responsiveness()), score))
        }
    }
}
