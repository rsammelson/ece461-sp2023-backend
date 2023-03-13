#[cfg(test)]
mod tests;

use crate::input::{Urls, Weights};

use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Parser)]
pub struct Cli {
    // This sets the Cli struct as well as how to address the variable flags
    pattern: String,
    #[clap(short = 'b', long = "busFactor", default_value = "1")]
    bus_factor: f64,
    #[clap(short = 'c', long = "correctness", default_value = "1")]
    correctness_factor: f64,
    #[clap(short = 'u', long = "rampUpTime", default_value = "1")]
    ramp_up_time: f64,
    #[clap(short = 'r', long = "responsiveness", default_value = "1")]
    responsiveness: f64,
    #[clap(short = 'l', long = "licenseCompatibility", default_value = "1")]
    license_compatibility: f64,
    #[clap(short = 'd', long = "fractionDependencies", default_value = "1")]
    fraction_dependencies: f64,
    #[clap(short = 'v', long = "fractionReviewed", default_value = "1")]
    fraction_reviewed: f64,
    #[clap(short = 't', long = "test")]
    test_mode: bool,
}

pub enum TestMode {
    Test,
    Normal,
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Could not open `{0}` because \"{1}\"")]
    FileOpenError(String, io::Error),
}

// This function is used to parse the arguments from the command line
pub fn get_inputs<I>(args: I) -> Result<(Weights, Urls, TestMode), Box<dyn Error + Send + Sync>>
where
    I: Iterator<Item = String>,
{
    let args = Cli::parse_from(args);

    let weights = Weights {
        bus_factor: args.bus_factor,
        correctness_factor: args.correctness_factor,
        ramp_up_time: args.ramp_up_time,
        responsiveness: args.responsiveness,
        license_compatibility: args.license_compatibility,
        fraction_dependencies: args.fraction_dependencies,
        fraction_reviewed: args.fraction_reviewed,
    };

    Ok((
        weights,
        Urls {
            urls: read_lines(args.pattern)?,
        },
        if args.test_mode {
            TestMode::Test
        } else {
            TestMode::Normal
        },
    ))
}

fn read_lines<P>(
    filename: P,
) -> Result<io::Lines<io::BufReader<File>>, Box<dyn Error + Send + Sync>>
where
    P: AsRef<Path> + Clone + Into<String>,
{
    let file = File::open(&filename)
        .map_err(|file_error| CliError::FileOpenError(filename.clone().into(), file_error))?;
    Ok(io::BufReader::new(file).lines())
}
