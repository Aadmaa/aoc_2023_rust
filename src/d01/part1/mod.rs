use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
/// Example:  
/// 
/// ```rust,ignore
/// cargo run -- -p d1 -t a -f "./data/d01/sample.txt"
/// cargo run -- -p d1 -t a -f "./data/d01/main.txt"
/// ````
pub fn d1(reader: BufReader<File>) -> Result<u32, String> {

    let mut result = 0;
    for line in reader.lines() {
        let line_val = line.map_err(|e| e.to_string())?;
        // println!("{}", line_val);
        result += get_line_val(&line_val).unwrap();
    }

    Ok(result)
}

fn get_line_val(line: &str) -> Result<u32, String> {
    let rex = Regex::new(r"[\D]").unwrap();
    let rezz = rex.replace_all(line, "".to_string());

    let first = rezz.chars().nth(0).unwrap();
    let last = rezz.chars().last().unwrap();

    let str = format!("{}{}", first, last);

    str.parse::<u32>().map_err(|e| e.to_string())
} 