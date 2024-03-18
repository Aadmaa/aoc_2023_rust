use crate::{Args, ProblemPart};
use std::fs::File;
use std::io::BufReader;

mod part1;
mod part2;
mod shared;

/// Day 05
pub fn run_d5(args: Args) -> Result<u64, String> {

    if !args.file.exists() {
        return Err(format!("File not found: {}", args.file.to_str().unwrap()));
    }

    let file = File::open(&args.file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    match args.part {
        ProblemPart::A => part1::d5(reader),
        ProblemPart::B => part2::d5_b(reader),
    }
}

#[cfg(test)]
mod tests {
    use crate::ProblemNumber;

    use super::*;

    #[test]
    fn test_d5() {
        let result = run_d5(Args::new(
            ProblemNumber::D5, 
            ProblemPart::A, 
            "data/d05/sample.txt"
        ));
    
        assert_eq!(result, Ok(35));
    }

    #[test]
    fn test_d5_solved() {
        let result = run_d5(Args::new(
            ProblemNumber::D5, 
            ProblemPart::A, 
            "data/d05/main.txt"
        ));
    
        assert_eq!(result, Ok(535088217));
    }

    #[test]
    fn test_d5_b() {
        let result = run_d5(Args::new(
            ProblemNumber::D5, 
            ProblemPart::B, 
            "data/d05/sample.txt"
        ));
    
        assert_eq!(result, Ok(46));
    }

    #[test]
    fn test_d5_b_solved() {
        let result = run_d5(Args::new(
            ProblemNumber::D5, 
            ProblemPart::B, 
            "data/d05/main.txt"
        ));
    
        assert_eq!(result, Ok(51399228));
    }
}


