use crate::{Args, ProblemPart};
use std::fs::File;
use std::io::BufReader;

mod part1;
// mod part2;
mod shared;

/// Day 06
/// cargo run -- -p d6 -t a -f "./data/d06/sample.txt"
pub fn run_d6(args: Args) -> Result<u64, String> {

    if !args.file.exists() {
        return Err(format!("File not found: {}", args.file.to_str().unwrap()));
    }

    let file = File::open(&args.file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    match args.part {
        ProblemPart::A => part1::d6(reader),
        ProblemPart::B => todo!(), // part2::d6_b(reader),
    }
}

#[cfg(test)]
mod tests {
    use crate::ProblemNumber;

    use super::*;

    #[test]
    fn test_d6() {
        let result = run_d6(Args::new(
            ProblemNumber::D6, 
            ProblemPart::A, 
            "data/d06/sample.txt"
        ));
    
        assert_eq!(result, Ok(288));
    }

    #[test]
    fn test_d6_solved() {
        let result = run_d6(Args::new(
            ProblemNumber::D6, 
            ProblemPart::A, 
            "data/d06/main.txt"
        ));
    
        assert_eq!(result, Ok(840336));
    }
}