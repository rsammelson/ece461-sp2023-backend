use crate::{
    api::graphql::Queryable,
    controller::{responsiveness, tests::get_fake_repository, Metric, Scorer},
};

use std::{error::Error, fmt::Display};
use tokio::sync::Mutex;

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !(($x - $y).abs() < $d || ($y - $x).abs() < $d) {
            panic!("{} not close enough to {}", $x, $y);
        }
    };
}

struct FakeRepoName {}

impl Display for FakeRepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "test/test")
    }
}

#[async_trait::async_trait]
impl Queryable for FakeRepoName {
    async fn query_license(&self) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        panic!("Unexpected call to query_license");
    }

    async fn query_responsiveness(
        &self,
    ) -> Result<
        Vec<Result<chrono::Duration, Box<dyn Error + Send + Sync>>>,
        Box<dyn Error + Send + Sync>,
    > {
        Ok(vec![
            Ok(chrono::Duration::days(2)),
            Ok(chrono::Duration::days(1)),
            Err(Box::new(crate::BackendError::Test)),
            Ok(chrono::Duration::hours(84)),
            Ok(chrono::Duration::hours(36)),
        ])
    }
}

#[tokio::test]
async fn responsiveness_basic() {
    let metric = Metric::Responsiveness(responsiveness::Responsiveness());
    let repo = Mutex::new(get_fake_repository());

    let repo_identifier = FakeRepoName {};
    let (_scored_metric, score) = metric.score(&repo, &repo_identifier).await.unwrap();

    assert_delta!(score, 0.6, 0.00001);
}
