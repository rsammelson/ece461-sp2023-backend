#[cfg(test)]
mod tests;

use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Correctness();

#[async_trait]
impl Scorer for Correctness {
    async fn score<Q>(
        &self,
        _repo: &Mutex<git2::Repository>,
        repo_identifier: &Q,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>>
    where
        Q: Queryable + fmt::Display + Sync + 'static,
    {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze Correctness for {repo_identifier}"),
        );

        Ok((Metric::Correctness(Correctness()), -1.))
    }
}
