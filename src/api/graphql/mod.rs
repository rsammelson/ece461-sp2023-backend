use super::fetch::GithubRepositoryName;
use crate::{
    api::{self, APIError},
    log::{self, LogLevel},
};

use self::responsiveness_query::{
    CommentAuthorAssociation, ResponsivenessQueryRepositoryIssuesNodes,
};

use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use serde::Deserialize;
use std::error::Error;

// #[allow(clippy::upper_case_acronyms)]
// type URI = String;

#[derive(Debug)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        let dt = chrono::DateTime::parse_from_rfc3339(s).map_err(serde::de::Error::custom)?;
        Ok(DateTime(dt.into()))
    }
}

#[allow(dead_code)]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/graphql/schema.json",
    query_path = "src/api/graphql/queries.graphql",
    response_derives = "Debug"
)]
struct LicenseQuery {
    owner: String,
    name: String,
}

#[allow(dead_code)]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/graphql/schema.json",
    query_path = "src/api/graphql/queries.graphql",
    response_derives = "Debug"
)]
struct ResponsivenessQuery {
    owner: String,
    name: String,
}

#[async_trait::async_trait]
pub trait Queryable {
    async fn query_license(&self) -> Result<Option<String>, Box<dyn Error + Send + Sync>>;
    async fn query_responsiveness(
        &self,
    ) -> Result<
        Vec<Result<chrono::Duration, Box<dyn Error + Send + Sync>>>,
        Box<dyn Error + Send + Sync>,
    >;
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
            .ok_or(APIError::NoReponse("querying license"))?;

        let repo = response_data.repository.ok_or(APIError::InvalidResponse(
            "missing repository while querying license",
        ))?;

        if let Some(license_info) = repo.license_info {
            if license_info.pseudo_license {
                Ok(None)
            } else {
                Ok(license_info.spdx_id)
            }
        } else {
            Ok(None)
        }
    }

    async fn query_responsiveness(
        &self,
    ) -> Result<
        Vec<Result<chrono::Duration, Box<dyn Error + Send + Sync>>>,
        Box<dyn Error + Send + Sync>,
    > {
        let client = api::get_github_client()?;

        let vars = responsiveness_query::Variables {
            owner: self.owner.to_owned(),
            name: self.name.to_owned(),
        };

        log::log(
            LogLevel::None,
            &format!(
                "Starting responsiveness query for {}/{}",
                &self.owner, &self.name
            ),
        );

        let response_body =
            post_graphql::<ResponsivenessQuery, _>(&client, "https://api.github.com/graphql", vars)
                .await?;

        let response_data = response_body
            .data
            .ok_or(APIError::NoReponse("querying license"))?;

        let issues = response_data
            .repository
            .and_then(|r| r.issues.nodes)
            .ok_or(APIError::InvalidResponse(
                "missing issue nodes while querying responsiveness",
            ))?;

        let times = issues.iter().flatten().map(issue_to_response_time);
        Ok(times.collect())
    }
}

fn issue_to_response_time(
    issue: &ResponsivenessQueryRepositoryIssuesNodes,
) -> Result<chrono::Duration, Box<dyn Error + Send + Sync>> {
    let start = &issue.created_at;
    let comments = issue
        .comments
        .nodes
        .as_ref()
        .ok_or(APIError::InvalidResponse(
            "issue to response time issue didn't have commend nodes",
        ))?;
    let end = comments
        .iter()
        .flatten()
        .find(|c| check_author_association(&c.author_association))
        .map(|c| &c.created_at);

    let now = DateTime(chrono::Utc::now());
    let mut end = match end {
        Some(t) => t,
        None => &now,
    };

    if issue.closed {
        if let Some(close_time) = &issue.closed_at {
            if close_time.0 < end.0 {
                end = close_time;
            }
        }
    }

    Ok(end.0 - start.0)
}

fn check_author_association(association: &CommentAuthorAssociation) -> bool {
    matches! {
        association,
        CommentAuthorAssociation::MEMBER
        | CommentAuthorAssociation::OWNER
        | CommentAuthorAssociation::COLLABORATOR
        | CommentAuthorAssociation::CONTRIBUTOR
    }
}
