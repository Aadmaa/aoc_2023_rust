use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::d03::shared::{invert_line_value, search_slice_with_regex};

/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d3 -t a -f "./data/d03/sample.txt"
/// cargo run -- -p d3 -t a -f "./data/d03/main.txt"
/// ````
pub fn d3(reader: BufReader<File>) -> Result<u32, String> {
    let mut result = 0;

    let padded_iter = std::iter::once(None)
        .chain(reader.lines().map(Some))
        .chain(std::iter::once(None));

    for window in padded_iter.collect::<Vec<_>>().windows(3) {
        let prev = invert_line_value(&window[0])?;
        let curr = invert_line_value(&window[1])?.unwrap(); 
        let next = invert_line_value(&window[2])?;

        result += get_line_val(&prev, &curr, &next);
    }

    println!("RESULT: {}", result);

    Ok(result)
}



fn get_line_val(prev: &Option<String>, curr: &str, next: &Option<String>) -> u32 {
    let re = Regex::new(r"[0-9]+").unwrap();
    let matches = re.find_iter(&curr);

    // let mut valid_numbers: Vec<u32> = vec![];
    let mut result: u32 = 0;

    for num in matches {
        let start = max(num.start().saturating_sub(1), 0);
        let end = min(num.end(), curr.len() - 1);

        let curr_as_num: u32 = num.as_str().parse().unwrap();

        let pattern = "[^0-9.]+";

        let curr_option: Option<String> = Some(curr.to_string());

        if search_slice_with_regex(prev, start, end, pattern)
            || search_slice_with_regex(&curr_option, start, end, pattern)
            || search_slice_with_regex(next, start, end, pattern)
        {
            result += curr_as_num;
        }
    }

    result
}
