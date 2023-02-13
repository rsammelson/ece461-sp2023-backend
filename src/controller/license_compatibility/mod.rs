#[cfg(test)]
mod tests;

use crate::{api::graphql::Queryable, controller::*};

use lazy_static::lazy_static;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct LicenseCompatibility();

const LICENSE_JSON_STR: &str = include_str!("licenses.json");

#[async_trait]
impl Scorer for LicenseCompatibility {
    async fn score(
        &self,
        _repo: &Mutex<git2::Repository>,
        repo_identifier: &GithubRepositoryName,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>> {
        lazy_static! {
            static ref LICENSE_JSON: serde_json::Value =
                serde_json::from_str(LICENSE_JSON_STR).unwrap();
            static ref LICENSE_MAP: &'static serde_json::Map<String, serde_json::Value> =
                LICENSE_JSON.as_object().unwrap();
            static ref LICENSE_GENERAL: &'static serde_json::Map<String, serde_json::Value> =
                LICENSE_MAP.get("general").unwrap().as_object().unwrap();
            static ref LICENSE_SPECIFIC: &'static serde_json::Map<String, serde_json::Value> =
                LICENSE_MAP
                    .get("license_specific")
                    .unwrap()
                    .as_object()
                    .unwrap();
        }

        log::log(
            LogLevel::All,
            &format!("Starting to analyze LicenseCompatibility for {repo_identifier}"),
        );

        let license = repo_identifier.query_license().await?;
        log::log(
            LogLevel::All,
            &format!("Got license {license:?} for {repo_identifier}"),
        );

        if let Some(l) = license {
            for a in LICENSE_GENERAL.values() {
                if let Some(score) = score_from_array(&l, a.as_array().unwrap()) {
                    return Ok((Metric::LicenseCompatibility(LicenseCompatibility()), score));
                }
            }

            let arr_specific = LICENSE_SPECIFIC
                .get("LGPL-2.1")
                .unwrap()
                .as_array()
                .unwrap();
            if let Some(score) = score_from_array(&l, arr_specific) {
                return Ok((Metric::LicenseCompatibility(LicenseCompatibility()), score));
            }
        }

        Ok((Metric::LicenseCompatibility(LicenseCompatibility()), 0.))
    }
}

fn score_from_array(license: &str, arr: &[serde_json::Value]) -> Option<f64> {
    let licenses = arr[1].as_array().unwrap();
    if licenses.iter().any(|s| s.as_str().unwrap() == license) {
        Some(arr[0].as_f64().unwrap())
    } else {
        None
    }
}
