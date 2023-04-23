pub mod url_conversion;

use crate::{log, log::LogLevel};
// use dirs;
use git2::Repository;
use std::env;
use std::{error::Error, fmt::Display, path::PathBuf};
use tokio::{fs, task};
use url;

#[derive(Debug)]
pub struct GithubRepositoryName {
    pub owner: String,
    pub name: String,
}

impl Display for GithubRepositoryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl GithubRepositoryName {
    pub fn as_url(&self) -> String {
        format!("https://github.com/{}/{}", self.owner, self.name)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryCreationError {
    #[error("Error while getting repository: `{0}`")]
    RepoError(&'static str),
    // #[error("Error while getting repository: `{0}`")]
    // OtherError(&'static str),
}

// CACHE PATH IS DEFINED HERE
// fn get_cache_dir() -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
//     Ok(dirs::cache_dir().ok_or(RepositoryCreationError::OtherError(
//         "Cannot locate home directory",
//     ))?)
// }

// Get the current working directory
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

/// Given a `Url`,
/// if a repository already exists at $HOME/.cache/acme/{user}/{repo},
/// - fetch from remote and update local state
///
/// if a local repository does not exist,
/// - clone it
pub async fn fetch_repo(
    project_url: url::Url,
) -> Result<(Repository, GithubRepositoryName), Box<dyn Error + Send + Sync>> {
    let repo = url_conversion::url_to_repo_name(project_url).await?;

    log::log(LogLevel::All, &format!("Starting update for {repo}"));

    let url = format!("https://github.com/{}/{}.git", repo.owner, repo.name);

    //Find operating system
    //let info = os_info::get();
    //let op_sys = info.os_type();

    //PATH SET HERE
    //let path = get_cache_dir()?.join("acme").join(&repo.owner);
    let path = get_current_working_dir()?.join("acme").join(&repo.owner);
    let repo_path = path.join(&repo.name);

    fs::create_dir_all(path).await?;

    // `repo` moved into return value
    let done_str = &format!("Done updating {repo}");

    let ret = match fs::metadata(repo_path.clone()).await {
        // directory does not exist
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok((
            task::spawn_blocking(move || Repository::clone(&url, repo_path)).await??,
            repo,
        )),
        Ok(m) if m.is_dir() => {
            // directory exists
            match task::spawn_blocking(move || update_repo(&url, repo_path)).await? {
                Ok(repo_local) => Ok((repo_local, repo)),
                Err(e) => Err(e),
            }
        }
        // lacking permissions
        Err(e) => return Err(Box::new(e)),
        Ok(_) => {
            return Err(Box::new(RepositoryCreationError::RepoError(
                "Repository clone location exists as a file",
            )))
        }
    };

    log::log(LogLevel::All, done_str);
    ret
}

fn update_repo(url: &str, repo_path: PathBuf) -> Result<Repository, Box<dyn Error + Send + Sync>> {
    let repo = Repository::open(repo_path)?;

    // check for changes
    if repo.state() != git2::RepositoryState::Clean {
        return Err(Box::new(RepositoryCreationError::RepoError(
            "Repository exists but is not clean",
        )));
    }

    let (mut remote, remote_name) = get_remote(&repo, url)?;

    // checkout main branch
    remote.connect(git2::Direction::Fetch)?;
    let default_branch = remote.default_branch()?;
    let default_branch_str = default_branch.as_str().ok_or_else(|| {
        Box::new(RepositoryCreationError::RepoError(
            "Default branch name was not valid UTF-8",
        ))
    })?;
    remote.fetch(&[default_branch_str], None, Some("acme tool"))?;

    let default_branch_name = default_branch_str
        .split_once('/')
        .unwrap()
        .1
        .split_once('/')
        .unwrap()
        .1;
    let remote_branch_name = format!("{remote_name}/{default_branch_name}");

    let remote_branch = repo.find_branch(&remote_branch_name, git2::BranchType::Remote)?;
    let remote_branch_head = repo.reference_to_annotated_commit(&remote_branch.into_reference())?;

    repo.set_head_detached_from_annotated(remote_branch_head)?;
    repo.checkout_head(None)?;

    drop(remote);
    Ok(repo)
}

fn get_remote<'repo>(
    repo: &'repo Repository,
    repo_identifier: &str,
) -> Result<(git2::Remote<'repo>, String), Box<dyn Error + Send + Sync>> {
    let mut remote = None;
    let mut remote_name = None;
    let remotes = repo.remotes()?;
    for rn in remotes.iter().flatten() {
        let remote_object = repo.find_remote(rn)?;
        if let Some(remote_url) = remote_object.url() {
            if remote_url == repo_identifier {
                // found remote
                remote = Some(remote_object);
                remote_name = Some(rn.to_owned());
                break;
            }
        }
    }
    if remote.is_none() {
        remote = Some(repo.remote("acme_tool_remote", repo_identifier)?);
        remote_name = Some("acme_tool_remote".to_owned());
    }
    Ok((remote.unwrap(), remote_name.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::GithubRepositoryName;

    #[test]
    fn github_repository_name_display() {
        let repo = GithubRepositoryName {
            owner: "owner".to_string(),
            name: "project".to_string(),
        };
        assert_eq!("owner/project", format!("{repo}"));
    }

    #[test]
    fn github_repository_name_url() {
        let repo = GithubRepositoryName {
            owner: "owner".to_string(),
            name: "project".to_string(),
        };
        assert_eq!("https://github.com/owner/project", repo.as_url());
    }
}
