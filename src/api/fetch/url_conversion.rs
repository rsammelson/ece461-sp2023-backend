use crate::{api, api::fetch::GithubRepositoryName, log, log::LogLevel};
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum URLConversionError {
    #[error("Error during URL conversion: error with URL: `{0}`")]
    URLError(&'static str),
    #[error("Error during URL conversion: error with API: `{0}`")]
    APIError(&'static str),
}

async fn http_to_repo_name(
    project_url: url::Url,
) -> Result<GithubRepositoryName, Box<dyn Error + Send + Sync>> {
    if let url::Host::Domain(s) = project_url.host().ok_or("URL must have a host")? {
        let get_err = || {
            Box::new(URLConversionError::URLError(
                "URL must be at least a second level domain name",
            ))
        };

        // get parts of FQDN from right to left
        let mut domain_parts = s.rsplit('.');

        // check that the TLD is com
        let tld = domain_parts.next().ok_or_else(get_err)?;
        match tld {
            "com" => {
                // get the second level domain name, check if it is npmjs or github
                let domain = domain_parts.next().ok_or_else(get_err)?;
                match domain {
                    "npmjs" => {
                        log::log(
                            LogLevel::Minimal,
                            &format!("Attempmting to convert {project_url} to a GitHub url"),
                        );
                        let client = api::get_client()?;
                        let github_url = npm_to_github_url(client, project_url).await?;
                        github_to_repo_name(github_url)
                    }
                    "github" => github_to_repo_name(project_url),
                    _ => Err(Box::new(URLConversionError::URLError(
                        "unrecognized domain name",
                    ))),
                }
            }
            _ => Err(Box::new(URLConversionError::URLError("unrecognized TLD"))),
        }
    } else {
        Err(Box::new(URLConversionError::URLError(
            "host must be a domain",
        )))
    }
}

async fn npm_to_github_url(
    client: reqwest::Client,
    mut project_url: url::Url,
) -> Result<url::Url, Box<dyn Error + Send + Sync>> {
    // ensure https
    if project_url.scheme() == "http" {
        project_url
            .set_scheme("https")
            .map_err(|_| Box::new(URLConversionError::URLError("could not set scheme")))?;
    }

    // get package name from URL
    let npm_path = project_url
        .path_segments()
        .ok_or_else(|| Box::new(URLConversionError::URLError("npm URLs must have a path")))?;

    let package_name = npm_path.last().ok_or_else(|| {
        Box::new(URLConversionError::URLError(
            "npm URL paths must have components",
        ))
    })?;

    // use the npm API to get the package information
    let response = client
        .get("https://registry.npmjs.org/".to_owned() + package_name)
        .send()
        .await?;

    // retrieve the response body as json
    let json = response.json::<serde_json::Value>().await?;

    // check the key "repository"
    let repo_obj = json
        .get("repository")
        .and_then(|o| o.as_object())
        .ok_or_else(|| {
            Box::new(URLConversionError::URLError(
                "npm response did not contain repository object",
            ))
        })?;

    let repo_type = repo_obj
        .get("type")
        .and_then(|s| s.as_str())
        .ok_or_else(|| {
            Box::new(URLConversionError::APIError(
                "npm response repository object did not contain type string",
            ))
        })?;

    // the type should be git
    if repo_type == "git" {
        // get the git URL
        let repo_url = repo_obj
            .get("url")
            .and_then(|s| s.as_str())
            .ok_or_else(|| {
                Box::new(URLConversionError::APIError(
                    "npm response repository object did not contain url string",
                ))
            })?;
        // parse it as a URL
        Ok(url::Url::parse(repo_url)?)
    } else {
        Err(Box::new(URLConversionError::URLError(
            "npm repository must be a git repository",
        )))
    }
}

fn github_to_repo_name(
    github_url: url::Url,
) -> Result<GithubRepositoryName, Box<dyn Error + Send + Sync>> {
    // the host should be a domain name, not an IP address
    if let url::Host::Domain(d) = github_url
        .host()
        .ok_or_else(|| Box::new(URLConversionError::URLError("GitHub URLs must have hosts")))?
    {
        // and the host should be GitHub
        if d.ends_with("github.com") {
            let mut repository = github_url.path_segments().ok_or_else(|| {
                Box::new(URLConversionError::URLError("GitHub URLs must have a path"))
            })?;

            let owner = repository
                .next()
                .ok_or("GitHub URLs must have a nonempty path")?;

            let segment_two = repository
                .next()
                .ok_or("GitHub URLs must have a multiple path segments")?;

            if let Some((name, _)) = segment_two.split_once('.') {
                Ok(GithubRepositoryName {
                    owner: owner.to_owned(),
                    name: name.to_owned(),
                })
            } else {
                Ok(GithubRepositoryName {
                    owner: owner.to_owned(),
                    name: segment_two.to_owned(),
                })
            }
        } else {
            Err(Box::new(URLConversionError::URLError(
                "URLs must be on GitHub",
            )))
        }
    } else {
        Err(Box::new(URLConversionError::URLError("URLs must domains")))
    }
}

pub async fn url_to_repo_name(
    project_url: url::Url,
) -> Result<GithubRepositoryName, Box<dyn Error + Send + Sync>> {
    match project_url.scheme() {
        "git" => github_to_repo_name(project_url),
        "http" | "https" => http_to_repo_name(project_url).await,
        _ => Err(Box::new(URLConversionError::URLError(
            "unrecognized URL scheme",
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::fetch::url_conversion::url_to_repo_name;

    #[tokio::test]
    async fn github_https_repo_convert() {
        let url = "https://github.com/facebook/react";
        let url = url::Url::parse(&url).unwrap();
        let repo = url_to_repo_name(url).await.unwrap();
        assert_eq!("facebook/react", format!("{repo}"));
    }

    #[tokio::test]
    async fn githuh_git_repo_convert() {
        let url = "git://github.com/jonschlinkert/even.git";
        let url = url::Url::parse(&url).unwrap();
        let repo = url_to_repo_name(url).await.unwrap();
        assert_eq!("jonschlinkert/even", format!("{repo}"));
    }

    // mock npm api response
    // #[tokio::test]
    // async fn npm_https_repo_convert() {
    //     let url = "https://www.npmjs.com/package/react-scripts";
    //     let url = url::Url::parse(&url).unwrap();
    //     let repo = url_to_repo_name(url).await.unwrap();
    //     assert_eq!("facebook/create-react-app", format!("{repo}"));
    // }
}
