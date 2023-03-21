#[cfg(test)]
mod tests;

use crate::controller::*;
// use std::process::Command;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FractionDependencies();

#[async_trait]
impl Scorer for FractionDependencies {
    //grab repo owner (arg1)
    //grab repo name (arg2)
    //pass to python code
    async fn score<Q>(
        &self,
        _repo: &Mutex<git2::Repository>,
        repo_identifier: &Q,
    ) -> Result<(Metric, f64), Box<dyn Error + Send + Sync>>
    where
        Q: Queryable + fmt::Display + Sync + 'static,
    {
        log::log(
            LogLevel::All,
            &format!("Starting to analyze FractionDependencies for {repo_identifier}"),
        );
        
        // let repo = repo.lock().await;

        // let output = -1.0;
        println!("{}", repo_identifier);

        let out = 1.0;

        // let out = Command::new("python3")
        //                 .arg("fracDep.py")
        //                 .arg("nothing")
        //                 .output()
        //                 .expect("file.py broke :(");
        println!("{:?}", out);
    //use output num of dependencies to calc score
    //save or return score 

        log::log(
            LogLevel::All,
            &format!("Done analyzing FractionDependencies for {repo_identifier}"),
        );


        Ok((
            Metric::FractionDependencies(FractionDependencies()), 
            (-1.0),
            // (1.0/out),
        ))
        
    }
}
