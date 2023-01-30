use std::fmt::Display;

#[derive(Debug)]
pub struct Scores {
    pub name: String,
    pub scores: Vec<Score>,
}

impl Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"{}\": [", self.name)?;

        let mut scores = self.scores.iter();
        if let Some(first) = scores.next() {
            write!(f, "{first}")?;
        }
        for score in scores {
            write!(f, ", {score}")?;
        }

        write!(f, "]}}")?;
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
        write!(f, "\"{}\": {}", self.metric, self.score)
    }
}
