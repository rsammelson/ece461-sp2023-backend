use tokio::sync::Mutex;

use crate::{
    api::{graphql::MockQueryable, GithubRepositoryName},
    controller::{license_compatibility, tests::get_fake_repository, Metric, Scorer},
};

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) {
            panic!();
        }
    };
}

#[tokio::test]
async fn license_simple() {
    let metric = Metric::LicenseCompatibility(license_compatibility::LicenseCompatibility());
    let repo = Mutex::new(get_fake_repository());

    let repo_identifier = GithubRepositoryName {
        owner: "test".to_string(),
        name: "test".to_string(),
    };

    let mut mock = MockQueryable::new();
    mock.expect_query_license()
        .times(1)
        .returning(move || Ok(Some("MIT".to_string())));

    let (_scored_metric, score) = metric.score(&repo, &repo_identifier).await.unwrap();

    assert_delta!(score, 0.95, 0.00001);
}
