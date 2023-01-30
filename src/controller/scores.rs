use std::fmt::Display;

#[derive(Debug)]
/// A struct to represent a variable number of scores
/// ```
/// use backend::controller::{Score, Scores};
/// let result = Scores {
///     name: "example_name".to_string(),
///     scores: vec![
///         Score {
///             metric: "Field 1".to_string(),
///             score: 1.1,
///         },
///         Score {
///             metric: "Field 2".to_string(),
///             score: 0.3,
///         },
///     ],
/// };
/// assert_eq!(
///     format!("{}", result),
///     r#"{"example_name": {"Field 1": 1.1, "Field 2": 0.3}}"#
/// );
/// ```
pub struct Scores {
    pub name: String,
    pub scores: Vec<Score>,
}

impl Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"{}": {{"#, self.name)?;

        let mut scores = self.scores.iter();
        if let Some(first) = scores.next() {
            write!(f, "{first}")?;
        }
        for score in scores {
            write!(f, ", {score}")?;
        }

        write!(f, "}}}}")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Score {
    pub metric: String,
    pub score: f64,
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}": {}"#, self.metric, self.score)
    }
}
