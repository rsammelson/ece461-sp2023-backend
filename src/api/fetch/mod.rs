use std::error::Error;
pub mod url_conversion;

use url;

pub async fn fetch_repo(project_url: url::Url) -> Result<(), Box<dyn Error>> {
    println!(
        "{:#?}",
        url_conversion::url_to_repo_name(project_url).await?
    );
    Ok(())
}
