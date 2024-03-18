
use std::fs::File;
use std::io::BufReader;


use crate::d06::shared::{count_ways_to_win, parse_file_b};


/// Seed transformation problems: individual seeds
/// 
/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d6 -t a -f "./data/d06/sample.txt"
/// cargo run -- -p d6 -t a -f "./data/d06/main.txt"
/// ````
pub fn d6_b(reader: BufReader<File>) -> Result<u64, String> {

    let race = parse_file_b(reader).map_err(|e| e.to_string())?;

    println!("Race: {:?}", race);

    let result: u64 = count_ways_to_win(&race).unwrap();
    
    Ok(result)
}

