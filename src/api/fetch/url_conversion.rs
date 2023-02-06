use crate::{api, api::fetch::GithubRepositoryName};
use serde_json;
use std::error::Error;
use std::fmt::Display;
use url;

#[derive(Debug)]
pub struct URLError(&'static str);

impl Display for URLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for URLError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

async fn http_to_repo_name(
    project_url: url::Url,
) -> Result<GithubRepositoryName, Box<dyn Error + Send + Sync>> {
    if let url::Host::Domain(s) = project_url.host().ok_or("URL must have a host")? {
        let get_err = || Box::new(URLError("URL must be at least a second level domain name"));

        // get parts of FQDN from right to left
        let mut domain_parts = s.rsplit('.');

        // check that the TLD is com
        if domain_parts.next().ok_or_else(get_err)? == "com" {
            // get the second level domain name, check if it is npmjs or github
            let domain = domain_parts.next().ok_or_else(get_err)?;
            if domain == "npmjs" {
                let client = api::get_client()?;
                let github_url = npm_to_github_url(client, project_url).await?;
                github_to_repo_name(github_url)
            } else if domain == "github" {
                github_to_repo_name(project_url)
            } else {
                Err(Box::new(URLError("unrecognized domain name")))
            }
        } else {
            Err(Box::new(URLError("unrecognized TLD")))
        }
    } else {
        Err(Box::new(URLError("host must be a domain")))
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
            .map_err(|_| Box::new(URLError("could not set scheme")))?;
    }

    // get package name from URL
    let npm_path = project_url
        .path_segments()
        .ok_or_else(|| Box::new(URLError("npm URLs must have a path")))?;

    let package_name = npm_path
        .last()
        .ok_or_else(|| Box::new(URLError("npm URL paths must have components")))?;

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
        .ok_or_else(|| Box::new(URLError("npm response did not contain repository object")))?;

    let repo_type = repo_obj
        .get("type")
        .and_then(|s| s.as_str())
        .ok_or_else(|| {
            Box::new(URLError(
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
                Box::new(URLError(
                    "npm response repository object did not contain url string",
                ))
            })?;
        // parse it as a URL
        Ok(url::Url::parse(repo_url)?)
    } else {
        Err(Box::new(URLError(
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
        .ok_or_else(|| Box::new(URLError("GitHub URLs must have hosts")))?
    {
        // and the host should be GitHub
        if d.ends_with("github.com") {
            let mut repository = github_url
                .path_segments()
                .ok_or_else(|| Box::new(URLError("GitHub URLs must have a path")))?;

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
            Err(Box::new(URLError("URLs must be on GitHub")))
        }
    } else {
        Err(Box::new(URLError("URLs must domains")))
    }
}

pub async fn url_to_repo_name(
    project_url: url::Url,
) -> Result<GithubRepositoryName, Box<dyn Error + Send + Sync>> {
    // let client = api::get_client();
    if project_url.scheme() == "git" {
        github_to_repo_name(project_url)
    } else if project_url.scheme() == "http" || project_url.scheme() == "https" {
        http_to_repo_name(project_url).await
    } else {
        Err(Box::new(URLError("unrecognized URL scheme")))
    }
}
