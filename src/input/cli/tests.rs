use super::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Output,
    str,
};

use assert_cmd::Command;

#[test]
fn test_read_file() {
    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let assert = cmd.arg("tests/input-urls").arg("-t").assert();

    let Output { stdout, .. } = assert.get_output();
    let stdout = str::from_utf8(&stdout).unwrap();

    print!("Got stdout as:\n```\n{stdout}\n```\n");

    let file = File::open(
        std::env::current_dir()
            .unwrap()
            .join("tests/input-urls")
            .to_str()
            .unwrap(),
    )
    .unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines.flatten() {
        println!("Looking for {}", line);
        assert!(stdout.contains(&line));
    }
}

#[test]
fn test_set_args() {
    let weights = Weights {
        bus_factor: 1.,
        correctness_factor: 1.1,
        ramp_up_time: 1.5,
        responsiveness: 0.3,
        license_compatibility: 0.,
    };

    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let assert = cmd
        .arg("tests/input-urls")
        .arg("-t")
        .args(&[
            "-b",
            &format!("{}", weights.bus_factor),
            "-c",
            &format!("{}", weights.correctness_factor),
            "-u",
            &format!("{}", weights.ramp_up_time),
            "-r",
            &format!("{}", weights.responsiveness),
            "-l",
            &format!("{}", weights.license_compatibility),
        ])
        .assert();

    let Output { stdout, .. } = assert.get_output();
    let stdout = str::from_utf8(&stdout).unwrap();

    print!("Got stdout as:\n```\n{stdout}\n```\n");

    assert!(stdout.contains(&format!("{weights}")));
}
