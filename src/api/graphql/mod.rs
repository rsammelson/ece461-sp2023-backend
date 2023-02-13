use crate::api::{self, APIError};
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

pub async fn graphql_query_license(
    repo: &GithubRepositoryName,
) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
    let client = api::get_github_client()?;

    let vars = license_query::Variables {
        owner: repo.owner.to_owned(),
        name: repo.name.to_owned(),
    };

    let response_body =
        post_graphql::<LicenseQuery, _>(&client, "https://api.github.com/graphql", vars).await?;

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
