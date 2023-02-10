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
    CorrectnessFactor,
    RampUpTime,
    Responsiveness,
    LicenseCompatibility,
}
//CorrectnessFactor(correctness_factor::CorrectnessFactor),
//RampUpTime(ramp_up_time::RampUpTime),
//Responsiveness(responsiveness::Responsiveness),
//LicenseCompatibility(license_compatibility::LicenseCompatibility),


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
            _ => Ok(-1.),
        }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Metric::BusFactor(_) => write!(f, "BusFactor"),
            Metric::CorrectnessFactor => write!(f, "CorrectnessFactor"),
            Metric::RampUpTime => write!(f, "RampUpTime"),
            Metric::Responsiveness => write!(f, "Responsiveness"),
            Metric::LicenseCompatibility => write!(f, "LicenseCompatibility"),
        }
    }
}

impl FromStr for Metric {
    type Err = ControllerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Metric::*;
        match s {
            "BusFactor" => Ok(BusFactor(bus_factor::BusFactor())),
            //below commented out lines to accomodate for the fact that the other metrics are not programmed yet.
            //"CorrectnessFactor" => Ok(CorrectnessFactor(correctness_factor::CorrectnessFactor())),
            //"RampUpTime" => Ok(RampUpTime(ramp_up_time::RampUpTime())),
            //"Responsiveness" => Ok(Responsiveness(responsiveness::Responsiveness())),
            //"LicenseCompatibility" => Ok(LicenseCompatibility(license_compatibility::LicenseCompatibility())),
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
        Metrics(vec![Metric::BusFactor(bus_factor::BusFactor()), Metric::CorrectnessFactor, Metric::RampUpTime, Metric::Responsiveness, Metric::LicenseCompatibility])
        //Metrics(vec![Metric::CorrectnessFactor(correctness_factor::CorrectnessFactor())])
        //Metrics(vec![Metric::RampUpTime(ramp_up_time::RampUpTime())])
        //Metrics(vec![Metric::Responsiveness(responsiveness::Responsiveness())])
        //Metrics(vec![Metric::LicenseCompatibility(license_compatibility::LicenseCompatibility())])
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
