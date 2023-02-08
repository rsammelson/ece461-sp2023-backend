use crate::input::{Urls, Weights};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pattern: String,
    #[clap(short = 'b', long = "busFactor", default_value = "1.0")]
    bus_factor: Option<f64>,
    #[clap(short = 'c', long = "correctness", default_value = "1.0")]
    correctness_factor: Option<f64>,
    #[clap(short = 't', long = "rampUpTime", default_value = "1.0")]
    ramp_up_time: Option<f64>,
    #[clap(short = 'r', long = "responsiveness", default_value = "1.0")]
    responsiveness: Option<f64>,
    #[clap(short = 'l', long = "licenseCompatibility", default_value = "1.0")]
    license_compatibility: Option<f64>,
    #[clap(short = 'd', long = "disableTests", default_value = "false")]
    disable_tests: Option<bool>,
}

pub fn get_inputs() -> (Weights, Urls) {
    let args = Cli::parse();

    let weights = Weights {
        bus_factor: args.bus_factor.unwrap_or(1.0),
        correctness_factor: args.correctness_factor.unwrap_or(1.0),
        ramp_up_time: args.ramp_up_time.unwrap_or(1.0),
        responsiveness: args.responsiveness.unwrap_or(1.0),
        license_compatibility: args.license_compatibility.unwrap_or(1.0),
    };

    let urls = Urls::new();

    (weights, urls)
}
