#[cfg(test)]
mod tests;

use crate::controller::*;
use std::process::Command;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FractionDependencies();

#[async_trait]
impl Scorer for FractionDependencies {
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
        
        //grab repo owner (arg1)
        //grab repo name (arg2)
        //pass to python code

        // println!("{}", repo_identifier);
        // for casting to a string see https://doc.rust-lang.org/rust-by-example/conversion/string.html
        let repo_id_str : String = repo_identifier.to_string();
        // println!("{}", repo_id_str);

        let out = Command::new("python3")
                        .arg("src/controller/fraction_dependencies/fracDep.py")
                        .arg(repo_id_str)
                        .output()
                        .expect("file.py broke :(");

        println!("{:?}", out);

        let out_str = String::from_utf8_lossy(&out.stdout);

        println!("{:?}", out_str);

        //let out_float = out_str.parse::<f32>()?;

        //let score_ = 1.0 / out_float;

        //println!("{:?}", score_);
    //use output num of dependencies to calc score
    //save or return score 

        log::log(
            LogLevel::All,
            &format!("Done analyzing FractionDependencies for {repo_identifier}"),
        );


        Ok((
            Metric::FractionDependencies(FractionDependencies()), 
            (-1.0),
            // ((1.0/out_float)).into(),
        ))
        
    }
}
