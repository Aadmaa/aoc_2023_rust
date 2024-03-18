use crate::{Args, ProblemPart};
use std::fs::File;
use std::io::BufReader;

mod part1;
mod part2;
mod shared;

/// Day 03
pub fn run_d3(args: Args) -> Result<u32, String> {

    if !args.file.exists() {
        return Err(format!("File not found: {}", args.file.to_str().unwrap()));
    }

    let file = File::open(&args.file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    match args.part {
        ProblemPart::A => part1::d3(reader),
        ProblemPart::B => part2::d3_b(reader),
    }
}

#[cfg(test)]
mod tests {
    use crate::ProblemNumber;

    use super::*;

    #[test]
    fn test_d3() {
        let result = run_d3(Args::new(
            ProblemNumber::D3, 
            ProblemPart::A, 
            "data/d03/sample.txt"
        ));
    
        assert_eq!(result, Ok(4361));
    }

    // 525119 correct
    #[test]
    fn test_d3_solved() {
        let result = run_d3(Args::new(
            ProblemNumber::D3, 
            ProblemPart::A, 
            "data/d03/main.txt"
        ));
    
        assert_eq!(result, Ok(525119));
    }

    #[test]
    fn test_d3_b() {
        let result = run_d3(Args::new(
            ProblemNumber::D3, 
            ProblemPart::B, 
            "data/d03/sample.txt"
        ));
    
        assert_eq!(result, Ok(467835));
    }


    // SOLVED: 76504829

}


