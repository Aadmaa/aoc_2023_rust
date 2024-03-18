// use regex::Regex;
// use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::shared::count_winners;


/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d4 -t a -f "./data/d04/sample.txt"
/// cargo run -- -p d4 -t a -f "./data/d04/main.txt"
/// ````
pub fn d4_b(reader: BufReader<File>) -> Result<u32, String> {
    let mut result = 0;

    let mut lookahead: Vec<u32> = vec![];

    for line in reader.lines() {
        if let Ok(curr) = line {
            if let Some(new_val) = get_line_val(&curr, &mut lookahead) {
                result += new_val;
            }
        }
    }

    println!("RESULT: {}", result);

    Ok(result)
}


// Pops the leftmost item if there is one, else returns 0
fn lpop(lookahead: &mut Vec<u32>) -> u32 {
    if lookahead.len() == 0 {
        return 0;
    }
    lookahead.remove(0)
}




fn get_line_val(line: &str, lookahead: &mut Vec<u32>) -> Option<u32> {

    // Pop the leftmost item from the lookahead and add 1
    // That's MY final value
    let my_final_value = lpop(lookahead) + 1;

    // Calculate the number of "wins" in this Card
    if let Some(counter) = count_winners(line)  {
        if counter > 0 {
            for index in 0..(counter as usize) {
                if lookahead.len() == index {
                    lookahead.push(my_final_value);
                } else {
                    lookahead[index] += my_final_value;
                }
            }            
        }
    }

    Some(my_final_value)
}