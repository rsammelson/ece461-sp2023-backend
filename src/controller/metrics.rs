use std::ops::Deref;

use crate::controller::*;

/// An enum that is used to tell `run_metrics()` what to run
///
/// For each added scoring algorithm, add a new variant that simply
/// contains the unit struct created to implement the `Scorer` trait
pub enum Metric {
    CountCommits(count_commits::CountCommits),
    BusFactor(bus_factor::BusFactor),
}

#[async_trait]
impl Scorer for Metric {
    async fn score<P: AsRef<Path> + Send>(
        &self,
        path: P,
        url: &str,
    ) -> Result<Score, Box<dyn Error>> {
        use Metric::*;
        match self {
            CountCommits(unit) => unit.score(path, url).await,
            BusFactor(unit) => unit.score(path, url).await,
        }
    }
}

impl FromStr for Metric {
    type Err = ControllerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Metric::*;
        match s {
            "CountCommits" => Ok(CountCommits(count_commits::CountCommits())),
            "BusFactor" => Ok(BusFactor(bus_factor::BusFactor())),
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

impl<T> TryFrom<Vec<T>> for Metrics
where
    T: TryInto<Metric, Error = ControllerError>,
{
    type Error = ControllerError;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        // couldn't get a `.collect()` thing working
        // kept having type errors -_-
        let mut ret = Vec::with_capacity(value.len());
        for metric in value {
            ret.push(metric.try_into()?);
        }
        Ok(Metrics(ret))
    }
}

impl Deref for Metrics {
    type Target = Vec<Metric>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
