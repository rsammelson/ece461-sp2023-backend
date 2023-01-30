pub mod fetch;
pub use fetch::GithubRepositoryName;

pub mod graphql;

use reqwest::{header, Client, ClientBuilder};
use std::error::Error;

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
