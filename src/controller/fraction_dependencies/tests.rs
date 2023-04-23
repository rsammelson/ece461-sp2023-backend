// use crate::{
//     api::graphql::Queryable,
//     controller::{license_compatibility, tests::get_fake_repository, Metric, Scorer},
// };

// use std::{error::Error, fmt::Display};
// use tokio::sync::Mutex;

// macro_rules! assert_delta {
//     ($x:expr, $y:expr, $d:expr) => {
//         if !(($x - $y).abs() < $d || ($y - $x).abs() < $d) {
//             panic!("{} not close enough to {}", $x, $y);
//         }
//     };
// }

// struct FakeRepoName {
//     license: &'static str,
// }

// impl Display for FakeRepoName {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "test/test")
//     }
// }

// #[async_trait::async_trait]
// impl Queryable for FakeRepoName {
//     async fn query_license(&self) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
//         Ok(Some(self.license.to_owned()))
//     }

//     async fn query_responsiveness(
//         &self,
//     ) -> Result<
//         Vec<Result<chrono::Duration, Box<dyn Error + Send + Sync>>>,
//         Box<dyn Error + Send + Sync>,
//     > {
//         panic!("Unexpected call to query_responsiveness");
//     }
// }

// #[tokio::test]
// async fn license_simple() {
//     let metric = Metric::LicenseCompatibility(license_compatibility::LicenseCompatibility());
//     let repo = Mutex::new(get_fake_repository());

//     let repo_identifier = FakeRepoName { license: "MIT" };
//     let (_scored_metric, score) = metric.score(&repo, &repo_identifier).await.unwrap();

//     assert_delta!(score, 0.95, 0.00001);
// }

// #[tokio::test]
// async fn license_hard() {
//     let metric = Metric::LicenseCompatibility(license_compatibility::LicenseCompatibility());
//     let repo = Mutex::new(get_fake_repository());

//     let repo_identifier = FakeRepoName {
//         license: "LGPL-2.0+",
//     };
//     let (_scored_metric, score) = metric.score(&repo, &repo_identifier).await.unwrap();

//     assert_delta!(score, 0.95, 0.00001);
// }

// #[tokio::test]
// async fn license_low() {
//     let metric = Metric::LicenseCompatibility(license_compatibility::LicenseCompatibility());
//     let repo = Mutex::new(get_fake_repository());

//     let repo_identifier = FakeRepoName {
//         license: "Apache-1.0",
//     };
//     let (_scored_metric, score) = metric.score(&repo, &repo_identifier).await.unwrap();

//     assert_delta!(score, 0.05, 0.00001);
// }

// #[tokio::test]
// async fn license_none() {
//     let metric = Metric::LicenseCompatibility(license_compatibility::LicenseCompatibility());
//     let repo = Mutex::new(get_fake_repository());

//     let repo_identifier = FakeRepoName { license: "" };
//     let (_scored_metric, score) = metric.score(&repo, &repo_identifier).await.unwrap();

//     assert_delta!(score, 0., 0.00001);
// }
