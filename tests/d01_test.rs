use assert_cmd::Command;
use predicates::prelude::*;

// Note, most tests run from the libraries, but we test one externally here
// to ensure it actually runs properly from the command line and parses 
// the needed parameters

#[test]
fn test_output() {
    let mut cmd = Command::cargo_bin("aoc_2023").unwrap();

    cmd.args(&["-p", "d1", "-t", "a", "-f", "./data/d01/sample.txt"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match("^142$").unwrap());
}