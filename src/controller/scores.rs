use super::Metric;
use crate::api::GithubRepositoryName;

use std::{collections::HashMap, fmt::Display};

/// A struct to represent a variable number of scores
pub struct Scores {
    pub url: GithubRepositoryName,
    pub net_score: f64,
    pub scores: HashMap<Metric, f64>,
}

impl Default for Scores {
    fn default() -> Self {
        Scores {
            url: GithubRepositoryName {
                owner: "user".to_string(),
                name: "project".to_string(),
            },
            net_score: 0.,
            scores: HashMap::new(),
        }
    }
}

impl Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"URL": "{}""#, self.url.as_url())?;
        write!(f, r#", "NET_SCORE": {:.3}"#, self.net_score)?;

        for (metric, score) in self.scores.iter() {
            write!(f, r#", "{metric}": {score:.3}"#)?;
        }

        write!(f, "}}")?;
        Ok(())
    }
}

#[test]
fn score_display_format() {
    use crate::{
        api::GithubRepositoryName,
        controller::{bus_factor, Metric, Scores},
    };
    let result = Scores {
        url: GithubRepositoryName {
            owner: "user".to_string(),
            name: "project".to_string(),
        },
        net_score: 0.8,
        scores: vec![(Metric::BusFactor(bus_factor::BusFactor()), 0.3)]
            .into_iter()
            .collect(),
    };
    assert_eq!(
        format!("{}", result),
        r#"{"URL": "https://github.com/user/project", "NET_SCORE": 0.800, "BUS_FACTOR_SCORE": 0.300}"#
    );
}
