use crate::controller::metrics;
use std::{collections::HashMap, fmt::Display};

/// A struct to represent a variable number of scores
/// ```
/// use backend::controller::Scores;
/// let result = Scores {
///     url: "example.url".to_string(),
///     net_score: 0.8,
///     scores: vec![("BusFactor".try_into().unwrap(), 0.3)]
///         .into_iter()
///         .collect(),
/// };
/// assert_eq!(
///     format!("{}", result),
///     r#"{"URL": "example.url", "NetScore": 0.8, "BusFactor": 0.3}"#
/// );
/// ```
pub struct Scores {
    pub url: String,
    pub net_score: f64,
    pub scores: HashMap<metrics::Metric, f64>,
}

impl Default for Scores {
    fn default() -> Self {
        Scores {
            url: "".to_string(),
            net_score: 0.,
            scores: HashMap::new(),
        }
    }
}

impl Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"URL": "{}""#, self.url)?;
        write!(f, r#", "NetScore": {}"#, self.net_score)?;

        for (metric, score) in self.scores.iter() {
            write!(f, r#", "{metric}": {score}"#)?;
        }

        write!(f, "}}")?;
        Ok(())
    }
}
