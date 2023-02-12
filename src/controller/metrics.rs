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
    async fn score<P: AsRef<Path> + Send>(
        &self,
        path: P,
        url: &GithubRepositoryName,
    ) -> Result<f64, Box<dyn Error + Send + Sync>> {
        use Metric::*;
        match self {
            BusFactor(unit) => unit.score(path, url).await,
            Correctness(unit) => unit.score(path, url).await,
            RampUpTime(unit) => unit.score(path, url).await,
            Responsiveness(unit) => unit.score(path, url).await,
            LicenseCompatibility(unit) => unit.score(path, url).await,
        }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Metric::BusFactor(_) => write!(f, "BusFactor"),
            Metric::Correctness(_) => write!(f, "Correctness"),
            Metric::RampUpTime(_) => write!(f, "RampUpTime"),
            Metric::Responsiveness(_) => write!(f, "Responsiveness"),
            Metric::LicenseCompatibility(_) => write!(f, "LicenseCompatibility"),
        }
    }
}

impl FromStr for Metric {
    type Err = ControllerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Metric::*;
        match s {
            "BusFactor" => Ok(BusFactor(bus_factor::BusFactor())),
            "Correctness" => Ok(Correctness(correctness::Correctness())),
            "RampUpTime" => Ok(RampUpTime(ramp_up_time::RampUpTime())),
            "Responsiveness" => Ok(Responsiveness(responsiveness::Responsiveness())),
            "LicenseCompatibility" => Ok(LicenseCompatibility(
                license_compatibility::LicenseCompatibility(),
            )),
            _ => Err(ControllerError::MetricParseError(s.to_string())),
        }
    }
}

impl TryFrom<&str> for Metric {
    type Error = ControllerError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
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

impl TryFrom<Vec<&str>> for Metrics {
    type Error = ControllerError;
    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let ret = value
            .iter()
            .map(|it| (*it).try_into())
            .collect::<Result<Vec<Metric>, ControllerError>>()?;
        Ok(Metrics(ret))
    }
}

impl Deref for Metrics {
    type Target = Vec<Metric>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
