use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::d03::shared::invert_line_value;
use super::shared::get_numbers_touching_index;

/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d3 -t a -f "./data/d03/sample.txt"
/// cargo run -- -p d3 -t a -f "./data/d03/main.txt"
/// ````
pub fn d3_b(reader: BufReader<File>) -> Result<u32, String> {
    let mut result = 0;

    let padded_iter = std::iter::once(None)
        .chain(reader.lines().map(Some))
        .chain(std::iter::once(None));

    for window in padded_iter.collect::<Vec<_>>().windows(3) {
        let prev = invert_line_value(&window[0])?;
        let curr = invert_line_value(&window[1])?.unwrap(); // This should not be None, so we can get the string
        let next = invert_line_value(&window[2])?;

        result += get_line_val(&prev, &curr, &next);
    }

    println!("RESULT: {}", result);

    Ok(result)
}


fn get_line_val(prev: &Option<String>, curr: &str, next: &Option<String>) -> u32 {
    // Find stars
    let re = Regex::new(r"\*+").unwrap();
    let matches = re.find_iter(&curr);

    let curr_option: Option<String> = Some(curr.to_string());

    let mut line_result: u32 = 0;

    for star in matches {
        let star_index = star.start();

        let mut touch_curr = get_numbers_touching_index(&curr_option, star_index);
        let mut touch_prev = get_numbers_touching_index(prev, star_index);
        let mut touch_next = get_numbers_touching_index(next, star_index);

        let mut valid_numbers: Vec<u32> = vec![];

        valid_numbers.append(&mut touch_curr);
        valid_numbers.append(&mut touch_prev);
        valid_numbers.append(&mut touch_next);

        if valid_numbers.len() == 2 {
            line_result += valid_numbers[0] * valid_numbers[1];
        }

    }

    line_result

}


