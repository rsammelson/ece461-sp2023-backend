use super::Metric;
use crate::api::GithubRepositoryName;

use std::{collections::HashMap, fmt::Display};

/// A struct to represent a variable number of scores
pub struct Scores {
    pub repo_identifier: GithubRepositoryName,
    pub net_score: f64,
    pub scores: HashMap<Metric, f64>,
}

impl Default for Scores {
    fn default() -> Self {
        Scores {
            repo_identifier: GithubRepositoryName {
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
        write!(f, r#"{{"URL": "{}""#, self.repo_identifier.as_url())?;
        write!(f, r#", "NET_SCORE": {:.3}"#, self.net_score)?;

        for (metric, score) in self.scores.iter() {
            write!(f, r#", "{metric}": {score:.3}"#)?;
        }

        write!(f, "}}")?;
        Ok(())
    }
}
