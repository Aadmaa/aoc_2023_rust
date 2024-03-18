
use std::fs::File;
use std::io::BufReader;


use super::shared::{parse_file, Race};


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

/// You can win when you press the button for B seconds, and B(D - B) > the record
/// So that gives a quadratic equation with a lower and upper bound. You have to be
/// Above the lower bound and below the upper bound
pub fn count_ways_to_win (race: &Race) -> Result<u64, String> {

    let duration = race.duration as f64;
    let record = race.record_distance as f64;

    let radical = (duration*duration - (4.0 * record)).sqrt();
    let result_upper = (duration + radical) / 2.0;
    let result_lower = (duration - radical) / 2.0;

    // It is a TIE with the existing record if result_upper is exactly an integer, so we'd need to subtract one to be faster
    let upper = if result_upper == result_upper.floor() {
        (result_upper - 1.0) as u64
    } else {
        result_upper.floor() as u64
    };

    let lower = if result_lower == result_lower.ceil() {
        (result_lower + 1.0) as u64
    } else {
        result_lower.ceil() as u64
    };

    // Victories are from lower to upper inclusive

    Ok(upper - lower + 1)
}