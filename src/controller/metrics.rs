use std::{fmt::Display, ops::Deref};

use crate::controller::*;

/// An enum that is used to tell `run_metrics()` what to run
///
/// For each added scoring metric, please update:
/// - this struct with variant that contains the unit struct
/// - `FromStr` for `Metric`
/// - `all()` for `Metrics`
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Metric {
    BusFactor(bus_factor::BusFactor),
    Correctness(correctness::Correctness),
    RampUpTime(ramp_up_time::RampUpTime),
    Responsiveness(responsiveness::Responsiveness),
    LicenseCompatibility(license_compatibility::LicenseCompatibility),
}

#[async_trait]
impl Scorer for Metric {
    async fn score(
        &self,
        repo: &Mutex<git2::Repository>,
        url: &GithubRepositoryName,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>> {
        use Metric::*;
        match self {
            BusFactor(unit) => unit.score(repo, url).await,
            Correctness(unit) => unit.score(repo, url).await,
            RampUpTime(unit) => unit.score(repo, url).await,
            Responsiveness(unit) => unit.score(repo, url).await,
            LicenseCompatibility(unit) => unit.score(repo, url).await,
        }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Metric::BusFactor(_) => write!(f, "BUS_FACTOR_SCORE"),
            Metric::Correctness(_) => write!(f, "CORRECTNESS_SCORE"),
            Metric::RampUpTime(_) => write!(f, "RAMP_UP_SCORE"),
            Metric::Responsiveness(_) => write!(f, "RESPONSIVE_MAINTAINER_SCORE"),
            Metric::LicenseCompatibility(_) => write!(f, "LICENSE_SCORE"),
        }
    }
}

pub struct Metrics(Vec<Metric>);

impl Metrics {
    pub fn all() -> Self {
        Metrics(vec![
            Metric::BusFactor(bus_factor::BusFactor()),
            Metric::Correctness(correctness::Correctness()),
            Metric::RampUpTime(ramp_up_time::RampUpTime()),
            Metric::Responsiveness(responsiveness::Responsiveness()),
            Metric::LicenseCompatibility(license_compatibility::LicenseCompatibility()),
        ])
    }
}

impl Deref for Metrics {
    type Target = Vec<Metric>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
