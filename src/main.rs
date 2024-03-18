use std::process;

/// Example:  
/// 
/// ```rust,ignore
/// // Day 1, Part A
/// cargo run -- -p d1 -t a -f "./data/d01/sample.txt"
/// cargo run -- -p d1 -t a -f "./data/d01/main.txt"
/// // Day 1, Part B
/// cargo run -- -p d1 -t b -f "./data/d01/sample_b.txt"
/// cargo run -- -p d1 -t b -f "./data/d01/main.txt"
/// ````
pub fn main() {
    let result = aoc_2023::hello();

    match result {
        Ok(val) => {
            print!("{}", val);
            process::exit(0)
        }, // Indicate success
        Err(_) => process::exit(1), // Indicate failure
    }
}

