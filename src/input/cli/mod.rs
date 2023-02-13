use crate::input::{Urls, Weights};
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
pub struct Cli {
    // This sets the Cli struct as well as how to address the variable flags
    pattern: String,
    #[clap(short = 'b', long = "busFactor", default_value = "1")]
    bus_factor: Option<f64>,
    #[clap(short = 'c', long = "correctness", default_value = "1")]
    correctness_factor: Option<f64>,
    #[clap(short = 't', long = "rampUpTime", default_value = "1")]
    ramp_up_time: Option<f64>,
    #[clap(short = 'r', long = "responsiveness", default_value = "1")]
    responsiveness: Option<f64>,
    #[clap(short = 'l', long = "licenseCompatibility", default_value = "1")]
    license_compatibility: Option<f64>,
    #[clap(short = 'd', long = "disableTests", default_value = "false")]
    disable_tests: Option<bool>,
}

// This function is used to parse the arguments from the command line
pub fn get_inputs() -> Result<(Weights, Urls), Box<dyn Error + Send + Sync>> {
    let args = Cli::parse();

    // This assigns the weights to the variables. the unwrap_or(1) sets a default weight
    let weights = Weights {
        bus_factor: args.bus_factor.unwrap_or(1.),
        correctness_factor: args.correctness_factor.unwrap_or(1.),
        ramp_up_time: args.ramp_up_time.unwrap_or(1.),
        responsiveness: args.responsiveness.unwrap_or(1.),
        license_compatibility: args.license_compatibility.unwrap_or(1.),
    };

    Ok((
        weights,
        Urls {
            urls: read_lines(args.pattern)?,
        },
    ))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
