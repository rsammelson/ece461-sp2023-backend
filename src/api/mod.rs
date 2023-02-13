pub mod fetch;
pub use fetch::GithubRepositoryName;

pub mod graphql;

use lazy_static::lazy_static;
use reqwest::{header, Client, ClientBuilder};
use std::error::Error;

#[derive(Debug, thiserror::Error)]
enum APIError {
    #[error("Must provide GITHUB_API_TOKEN environment variable")]
    GithubTokenError,
    #[error("API did not receive a response while {0}")]
    NoReponseError(&'static str),
    #[error("API recieved an invalid response: {0}")]
    InvalidResponseError(&'static str),
}

fn get_client_builder() -> ClientBuilder {
    Client::builder()
        .user_agent("ece461-backend")
        .https_only(true)
}

pub fn get_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    Ok(get_client_builder().build()?)
}

pub fn get_github_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    lazy_static! {
        static ref TOKEN: Result<String, APIError> = Ok("bearer ".to_owned()
            + &std::env::var("GITHUB_API_TOKEN").map_err(|_| APIError::GithubTokenError)?);
    }

    let token = TOKEN.as_ref()?;

    let mut headers = header::HeaderMap::new();
    let mut token_header = header::HeaderValue::from_str(&token)?;
    token_header.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, token_header);

    Ok(get_client_builder().default_headers(headers).build()?)
}
