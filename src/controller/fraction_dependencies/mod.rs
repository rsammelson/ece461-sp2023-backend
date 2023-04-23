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
        let repo_id_str: String = repo_identifier.to_string();
        let out = Command::new("python3")
            .arg("src/controller/fraction_dependencies/frac_dep.py")
            .arg(repo_id_str)
            .output()
            .expect("file.py broke :(");

        //println!("{:?}", out);

        let out_str = String::from_utf8_lossy(&out.stdout);
        let final_str = out_str.trim();

        let out_float = final_str.parse::<f32>()?;
        let score_ = 1.0 / out_float;

        log::log(
            LogLevel::All,
            &format!(
                "Done analyzing FractionDependencies for {repo_identifier} with score {score_}."
            ),
        );

        Ok((
            Metric::FractionDependencies(FractionDependencies()),
            (score_).into(),
        ))
    }
}
