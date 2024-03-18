use crate::{Args, ProblemPart};
use std::fs::File;
use std::io::BufReader;

mod part1;
mod part2;

/// Day 01
pub fn run_d1(args: Args) -> Result<u32, String> {

    if !args.file.exists() {
        return Err(format!("File not found: {}", args.file.to_str().unwrap()));
    }

    let file = File::open(&args.file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    match args.part {
        ProblemPart::A => part1::d1(reader),
        ProblemPart::B => part2::d1_b(reader),
    }
}

/*
    cargo run -- -p d1 -t b -f "./data/d01/main.txt"
    54438
    That's not the right answer; your answer is too high.

    54431
    The bug: "eightwo" needs to be 82
    so you can't just replace even the 1st and last
*/

// region: Tests
#[cfg(test)]
mod tests {
    use crate::ProblemNumber;

    use super::*;

    #[test]
    fn test_d1() {

        let result = run_d1(Args::new(
            ProblemNumber::D1, 
            crate::ProblemPart::A,
            "data/d01/sample.txt"
        ));

        assert_eq!(result, Ok(142));
    }

    #[test]
    fn test_d1_solved() {

        let result = run_d1(Args::new(
            ProblemNumber::D1, 
            crate::ProblemPart::A,
            "data/d01/main.txt"
        ));

        assert_eq!(result, Ok(55477));
    }

    #[test]
    fn test_d1_b() {

        let result = run_d1(Args::new(
            ProblemNumber::D1, 
            crate::ProblemPart::B,
            "data/d01/sample_b.txt"
        ));

        assert_eq!(result, Ok(281));
    }

    #[test]
    fn test_d1_b_solved() {

        let result = run_d1(Args::new(
            ProblemNumber::D1, 
            crate::ProblemPart::B,
            "data/d01/main.txt"
        ));

        assert_eq!(result, Ok(54431));
    }

}

// endregion: Tests