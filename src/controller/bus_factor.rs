use crate::controller::*;

use std::collections::HashMap;

pub struct BusFactor();

#[async_trait]
impl Scorer for BusFactor {
    async fn score<P: AsRef<Path> + Send>(
        &self,
        path: P,
        _url: &str,
    ) -> Result<Score, Box<dyn Error>> {
        let repo = git2::Repository::open(path)?;

        let mut walk = repo.revwalk().unwrap();
        walk.set_sorting(git2::Sort::TIME)?;
        walk.push_head()?;

        let mut authors = HashMap::<String, u32>::new();

        for (res, _) in walk.into_iter().zip(0..500) {
            match res {
                Ok(oid) => {
                    let commit = repo.find_commit(oid)?;
                    let author = commit.author();
                    let name = author.name().map_or("unknown", |a| a);
                    let count = authors.get(name).map_or(0, |c| *c);
                    authors.insert(name.to_string(), count + 1);
                }
                Err(_) => (),
            }
        }

        for (author, number) in authors {
            println!("{}: {}", author, number);
        }

        Ok(Score {
            metric: "BusFactor".to_string(),
            score: 0.,
        })
    }
}
