pub mod fetch;

use reqwest::{Client, ClientBuilder};
use std::error::Error;

fn get_client_builder() -> ClientBuilder {
    Client::builder()
        .user_agent("ece461-backend")
        .https_only(true)
}

pub fn get_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    Ok(get_client_builder().build()?)
}
