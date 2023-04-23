pub mod cli;

use std::{fmt::Display, fs::File, io};

pub struct Weights {
    pub bus_factor: f64,
    pub correctness_factor: f64,
    pub ramp_up_time: f64,
    pub responsiveness: f64,
    pub license_compatibility: f64,
    pub fraction_dependencies: f64,
    pub fraction_reviewed: f64,
}

pub struct Urls {
    urls: io::Lines<io::BufReader<File>>,
}

impl Iterator for Urls {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.urls.next()
    }
}

#[cfg(test)]
impl Weights {
    pub fn new() -> Self {
        Weights {
            bus_factor: 0.,
            correctness_factor: 0.,
            ramp_up_time: 0.,
            responsiveness: 0.,
            license_compatibility: 0.,
            fraction_dependencies: 0.,
            fraction_reviewed: 0.,
        }
    }
}

impl Default for Weights {
    fn default() -> Self {
        Weights {
            bus_factor: 1.,
            correctness_factor: 1.,
            ramp_up_time: 1.,
            responsiveness: 1.,
            license_compatibility: 1.,
            fraction_dependencies: 1.,
            fraction_reviewed: 1.,
        }
    }
}

impl Display for Weights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BUS_FACTOR_WEIGHT: {}", self.bus_factor)?;
        writeln!(f, "CORRECTNESS_WEIGHT: {}", self.correctness_factor)?;
        writeln!(f, "RAMP_UP_WEIGHT: {}", self.ramp_up_time)?;
        writeln!(f, "RESPONSIVE_MAINTAINER_WEIGHT: {}", self.responsiveness)?;
        write!(f, "LICENSE_WEIGHT: {}", self.license_compatibility)?;
        write!(
            f,
            "FRACTION_DEPENDENCY_WEIGHT: {}",
            self.fraction_dependencies
        )?;
        write!(f, "FRACTION_REVIEWED_WEIGHT: {}", self.fraction_reviewed)?;
        Ok(())
    }
}
