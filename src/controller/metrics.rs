use std::ops::Deref;

use crate::controller::*;

pub enum Metric {
    CountCommits(count_commits::CountCommits),
    CountCommits2(count_commits::CountCommits2),
}

#[async_trait]
impl Scorer for Metric {
    async fn score<P: AsRef<Path> + Send>(&self, path: P, url: &str) -> Score {
        use Metric::*;
        match self {
            CountCommits(unit) => unit.score(path, url).await,
            CountCommits2(unit) => unit.score(path, url).await,
        }
    }
}

impl FromStr for Metric {
    type Err = ControllerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Metric::*;
        match s {
            "CountCommits" => Ok(CountCommits(count_commits::CountCommits())),
            "CountCommits2" => Ok(CountCommits2(count_commits::CountCommits2())),
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
