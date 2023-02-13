use crate::{
    api::{self, APIError},
    log::{self, LogLevel},
};
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use std::error::Error;

use super::fetch::GithubRepositoryName;

// #[allow(clippy::upper_case_acronyms)]
// type URI = String;
// type DateTime = String;

#[allow(dead_code)]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/graphql/schema.json",
    query_path = "src/api/graphql/license.graphql",
    response_derives = "Debug"
)]
struct LicenseQuery {
    owner: String,
    name: String,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait Queryable {
    async fn query_license(&self) -> Result<Option<String>, Box<dyn Error + Send + Sync>>;
}

#[async_trait::async_trait]
impl Queryable for GithubRepositoryName {
    async fn query_license(&self) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        let client = api::get_github_client()?;

        let vars = license_query::Variables {
            owner: self.owner.to_owned(),
            name: self.name.to_owned(),
        };

        log::log(
            LogLevel::None,
            &format!("Starting license query for {}/{}", &self.owner, &self.name),
        );

        let response_body =
            post_graphql::<LicenseQuery, _>(&client, "https://api.github.com/graphql", vars)
                .await?;

        let response_data = response_body
            .data
            .ok_or(APIError::NoReponseError("querying license"))?;

        let repo = response_data
            .repository
            .ok_or(APIError::InvalidResponseError(
                "missing repository while querying license",
            ))?;

        let license_info = repo.license_info.ok_or(APIError::InvalidResponseError(
            "missing license_info while querying license",
        ))?;

        if license_info.pseudo_license {
            return Ok(None);
        }

        Ok(license_info.spdx_id)
    }
}
