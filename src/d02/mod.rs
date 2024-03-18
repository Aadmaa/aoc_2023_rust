use crate::{Args, ProblemPart};
use std::fs::File;
use std::io::BufReader;

mod part1;
mod part2;
mod shared;

/// Day 02
pub fn run_d2(args: Args) -> Result<u32, String> {

    if !args.file.exists() {
        return Err(format!("File not found: {}", args.file.to_str().unwrap()));
    }

    let file = File::open(&args.file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    match args.part {
        ProblemPart::A => part1::d2(reader),
        ProblemPart::B => part2::d2_b(reader),
    }
}

#[cfg(test)]
mod tests {
    use crate::ProblemNumber;

    use super::*;

    #[test]
    fn test_d2() {
        let result = run_d2(Args::new(
            ProblemNumber::D2, 
            ProblemPart::A, 
            "data/d02/sample.txt"
        ));
    
        assert_eq!(result, Ok(8));
    }


    #[test]
    fn test_d2_solved() {
        let result = run_d2(Args::new(
            ProblemNumber::D2, 
            ProblemPart::A, 
            "data/d02/main.txt"
        ));
    
        assert_eq!(result, Ok(2593));
    }

    #[test]
    fn test_d2_b() {
        let result = run_d2(Args::new(
            ProblemNumber::D2, 
            ProblemPart::B, 
            "data/d02/sample.txt"
        ));
    
        assert_eq!(result, Ok(2286));
    }

    #[test]
    fn test_d2_b_solved() {
        let result = run_d2(Args::new(
            ProblemNumber::D2, 
            ProblemPart::B, 
            "data/d02/main.txt"
        ));
    
        assert_eq!(result, Ok(54699));
    }

    // 54699
    // 2286

}


