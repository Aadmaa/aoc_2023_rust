
use std::fs::File;
use std::io::BufReader;


use crate::d06::shared::count_ways_to_win;

use super::shared::parse_file;


/// Seed transformation problems: individual seeds
/// 
/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d6 -t a -f "./data/d06/sample.txt"
/// cargo run -- -p d6 -t a -f "./data/d06/main.txt"
/// ````
pub fn d6(reader: BufReader<File>) -> Result<u64, String> {

    let races = parse_file(reader).map_err(|e| e.to_string())?;

    println!("Races: {:?}", races);

    let result: u64 = races.iter().fold(1, | acc, race | acc * count_ways_to_win(race).unwrap());

    Ok(result)
}

