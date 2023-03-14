#[cfg(test)]
mod tests;

use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Correctness();

#[async_trait]
impl Scorer for FractionDependencies {
    //grab repo owner (arg1)
    //grab repo name (arg2)
    //pass to python code
    use std::process::Command;
    let output = Command::new("python3")
                        .arg("fracDep.py")
                        .arg("input1")
                        .arg("input2")
                        .output()
                        .expect("file.py broke :(");
    //use output num of dependencies to calc score
    //save or return score 

    Ok((Metric::FractionDependencies(FractionDependencies()), (1/output.output)))
}
