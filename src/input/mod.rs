pub mod cli;

use std::{fs::File, io};

pub struct Weights {
    pub bus_factor: f64,
    pub correctness_factor: f64,
    pub ramp_up_time: f64,
    pub responsiveness: f64,
    pub license_compatibility: f64,
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

impl Weights {
    pub fn new() -> Self {
        Weights {
            bus_factor: 0.,
            correctness_factor: 0.,
            ramp_up_time: 0.,
            responsiveness: 0.,
            license_compatibility: 0.,
        }
    }
}

impl Default for Weights {
    fn default() -> Self {
        Self::new()
    }
}
