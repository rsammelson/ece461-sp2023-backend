use crate::api;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use std::error::Error;

#[allow(clippy::upper_case_acronyms)]
type URI = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/graphql/schema.json",
    query_path = "src/api/graphql/test.graphql",
    response_derives = "Debug"
)]
struct TestQuery;

pub async fn graphql_query_test() -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = api::get_github_client()?;

    let vars = test_query::Variables {};

    // let body = TestQuery::build_query(vars);
    // let response = client.get("https://api.github.com/graphql").json(&body).send().await?;
    // let response_body: test_query::ResponseData = response.json().await?;

    let response_body =
        post_graphql::<TestQuery, _>(&client, "https://api.github.com/graphql", vars).await?;

    // println!("{:#?}", response);
    println!("{response_body:#?}");

    Ok(())
}
