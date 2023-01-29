pub mod fetch;
pub use fetch::GithubRepositoryName;

use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::{header, Client, ClientBuilder};
use std::error::Error;

#[allow(clippy::upper_case_acronyms)]
type URI = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema.json",
    query_path = "src/api/test.graphql",
    response_derives = "Debug"
)]
struct TestQuery;

fn get_client_builder() -> ClientBuilder {
    Client::builder()
        .user_agent("ece461-backend")
        .https_only(true)
}

pub fn get_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    Ok(get_client_builder().build()?)
}

pub fn get_github_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    let token = "bearer ".to_owned()
        + &std::env::var("GITHUB_API_TOKEN")
            .expect("Must provide GITHUB_API_TOKEN environment variable");

    let mut headers = header::HeaderMap::new();
    let mut token_header = header::HeaderValue::from_str(&token)?;
    token_header.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, token_header);

    Ok(get_client_builder().default_headers(headers).build()?)
}

pub async fn query_test() -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = get_github_client()?;

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
