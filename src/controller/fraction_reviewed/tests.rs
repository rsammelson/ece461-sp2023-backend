// use crate::{
//     api::GithubRepositoryName,
//     controller::{fraction_reviewed, tests::get_fake_repository, Metric, Scorer},
// };

// use tokio::sync::Mutex;

// macro_rules! assert_delta {
//     ($x:expr, $y:expr, $d:expr) => {
//         if !(($x - $y).abs() < $d || ($y - $x).abs() < $d) {
//             panic!("{} not close enough to {}", $x, $y);
//         }
//     };
// }

// #[tokio::test]
// async fn fraction_reviewed() {
//     let metric = Metric::FractionReviewed(fraction_reviewed::FractionReviewed());
//     let repo = Mutex::new(get_fake_repository());

//     let repo_identifier = GithubRepositoryName {
//         owner: "test".to_owned(),
//         name: "test".to_owned(),
//     };
//     let (_scored_metric, score) = metric.score(&repo, &repo_identifier).await.unwrap();

//     assert_delta!(score, -1., 0.00001);
// }
