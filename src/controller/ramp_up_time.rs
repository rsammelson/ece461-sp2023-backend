use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct RampUpTime();

#[async_trait]
impl Scorer for RampUpTime {
    async fn score<Q>(
        &self,
        repo: &Mutex<git2::Repository>,
        repo_identifier: &Q,
    ) -> Result<(Metric, f64), Box<dyn std::error::Error + Send + Sync>>
    where
        Q: Queryable + fmt::Display + Sync + 'static,
    {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze RampUpTime for {repo_identifier}"),
        );

        let repo = repo.lock().await;

        let folder = repo.path().parent().ok_or(crate::BackendError::Repo(
            "could not get repository location",
        ))?;

        let paths = std::fs::read_dir(folder)?;
        let filtered_paths = paths
            .filter_map(|f| f.ok())
            .filter(|f| f.metadata().ok().map(|m| m.is_file()).unwrap_or(false))
            .filter(|f| {
                f.file_name()
                    .to_ascii_uppercase()
                    .to_str()
                    .map(|n| n.contains("README"))
                    .unwrap_or(false)
            });

        let sizes = filtered_paths.map(|f| f.metadata().ok().map(|m| m.len()).unwrap_or(0));
        let size = ((sizes.sum::<u64>() + 1) as f64) / 1000.;
        let score = (size.log2() / 7.).clamp(0., 1.);

        drop(repo);

        log::log(
            LogLevel::All,
            &format!("Got ramp up {score} for {repo_identifier}"),
        );

        Ok((Metric::RampUpTime(RampUpTime()), score))
    }
}
