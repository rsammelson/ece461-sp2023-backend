pub mod url_conversion;

use dirs;
use git2::Repository;
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;
use tokio::fs;
use tokio::task;
use url;

#[derive(Debug)]
pub struct GithubRepositoryName {
    pub owner: String,
    pub name: String,
}

#[derive(Debug)]
struct RepoError {
    message: String,
}

impl Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.message))
    }
}

impl Error for RepoError {
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

fn get_cache_dir() -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    Ok(dirs::cache_dir().ok_or_else(|| RepoError {
        message: "Cannot locate home directory".to_owned(),
    })?)
}

fn update_repo(url: &str, repo_path: PathBuf) -> Result<Repository, Box<dyn Error + Send + Sync>> {
    let repo = Repository::open(repo_path)?;

    // check for changes
    if repo.state() != git2::RepositoryState::Clean {
        return Err(Box::new(RepoError {
            message: "Repository exists but is not clean".to_owned(),
        }));
    }

    // get the remote
    let mut remote = None;
    let mut remote_name = None;
    let remotes = repo.remotes()?;
    for rn in remotes.iter().flatten() {
        let remote_object = repo.find_remote(rn)?;
        if let Some(remote_url) = remote_object.url() {
            if remote_url == url {
                // found remote
                remote = Some(remote_object);
                remote_name = Some(rn);
                break;
            }
        }
    }
    if remote.is_none() {
        remote = Some(repo.remote("acme_tool_remote", url)?);
        remote_name = Some("acme_tool_remote");
    }
    let mut remote = remote.unwrap();
    let remote_name = remote_name.unwrap();

    // checkout main branch
    remote.connect(git2::Direction::Fetch)?;
    let default_branch = remote.default_branch()?;
    let default_branch_str = default_branch.as_str().ok_or_else(|| {
        Box::new(RepoError {
            message: "Default branch name was not valid UTF-8".to_owned(),
        })
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
        Ok(_) => Err(Box::new(RepoError {
            message: "Repository clone location exists as a file".to_owned(),
        })),
    }
}
