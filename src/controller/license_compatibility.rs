use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct LicenseCompatibility();

#[async_trait]
impl Scorer for LicenseCompatibility {
    async fn score<P: AsRef<Path> + Send>(
        &self,
        _path: P,
        url: &GithubRepositoryName,
    ) -> Result<f64, Box<dyn Error + Send + Sync>> {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze LicenseCompatibility for {url}"),
        );

        Ok(0.)
    }
}
