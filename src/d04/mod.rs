use crate::{Args, ProblemPart};
use std::fs::File;
use std::io::BufReader;

mod part1;
mod part2;
mod shared;

/// Day 04
pub fn run_d4(args: Args) -> Result<u32, String> {

    if !args.file.exists() {
        return Err(format!("File not found: {}", args.file.to_str().unwrap()));
    }

    let file = File::open(&args.file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    match args.part {
        ProblemPart::A => part1::d4(reader),
        ProblemPart::B => part2::d4_b(reader),
    }
}

#[cfg(test)]
mod tests {
    use crate::ProblemNumber;

    use super::*;

    #[test]
    fn test_d4() {
        let result = run_d4(Args::new(
            ProblemNumber::D4, 
            ProblemPart::A, 
            "data/d04/sample.txt"
        ));
    
        assert_eq!(result, Ok(13));
    }

    #[test]
    fn test_d4_solved() {
        let result = run_d4(Args::new(
            ProblemNumber::D4, 
            ProblemPart::A, 
            "data/d04/main.txt"
        ));
    
        assert_eq!(result, Ok(25174));
    }

    #[test]
    fn test_d4_b() {
        let result = run_d4(Args::new(
            ProblemNumber::D4, 
            ProblemPart::B, 
            "data/d04/sample.txt"
        ));
    
        assert_eq!(result, Ok(30));
    }

    #[test]
    fn test_d4_b_solved() {
        let result = run_d4(Args::new(
            ProblemNumber::D4, 
            ProblemPart::B, 
            "data/d04/main.txt"
        ));
    
        assert_eq!(result, Ok(6420979));
    }

}


