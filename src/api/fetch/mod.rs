pub mod url_conversion;

use dirs;
use git2::Repository;
use std::error::Error;
use std::path::PathBuf;
use tokio::fs;
use tokio::task;
use url;

#[derive(Debug)]
pub struct GithubRepositoryName {
    pub owner: String,
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryCreationError {
    #[error("Error while getting repository: `{0}`")]
    RepoError(&'static str),
    #[error("Error while getting repository: `{0}`")]
    OtherError(&'static str),
}

fn get_cache_dir() -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    Ok(dirs::cache_dir().ok_or(RepositoryCreationError::OtherError(
        "Cannot locate home directory",
    ))?)
}

pub async fn fetch_repo(project_url: url::Url) -> Result<Repository, Box<dyn Error + Send + Sync>> {
    let repo = url_conversion::url_to_repo_name(project_url).await?;

    let url = format!("https://github.com/{}/{}.git", repo.owner, repo.name);

    let path = get_cache_dir()?.join("acme").join(repo.owner);
    let repo_path = path.join(repo.name);

    fs::create_dir_all(path).await?;

    match fs::metadata(repo_path.clone()).await {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // directory does not exist
            Ok(task::spawn_blocking(move || Repository::clone(&url, repo_path)).await??)
        }
        Ok(m) if m.is_dir() => {
            // directory exists
            match task::spawn_blocking(move || update_repo(&url, repo_path)).await? {
                Ok(repo) => Ok(repo),
                Err(e) => Err(e),
            }
        }
        Err(e) => {
            // lacking permissions
            Err(Box::new(e))
        }
        Ok(_) => Err(Box::new(RepositoryCreationError::RepoError(
            "Repository clone location exists as a file",
        ))),
    }
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
    url: &str,
) -> Result<(git2::Remote<'repo>, String), Box<dyn Error + Send + Sync>> {
    let mut remote = None;
    let mut remote_name = None;
    let remotes = repo.remotes()?;
    for rn in remotes.iter().flatten() {
        let remote_object = repo.find_remote(rn)?;
        if let Some(remote_url) = remote_object.url() {
            if remote_url == url {
                // found remote
                remote = Some(remote_object);
                remote_name = Some(rn.to_owned());
                break;
            }
        }
    }
    if remote.is_none() {
        remote = Some(repo.remote("acme_tool_remote", url)?);
        remote_name = Some("acme_tool_remote".to_owned());
    }
    Ok((remote.unwrap(), remote_name.unwrap()))
}
