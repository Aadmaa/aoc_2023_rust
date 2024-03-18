use std::fs::File;
use std::io::{BufRead, BufReader};

use super::shared::count_winners;


/// Example:  
///
/// ```rust,ignore
/// cargo run -- -p d4 -t a -f "./data/d04/sample.txt"
/// cargo run -- -p d4 -t a -f "./data/d04/main.txt"
/// ````
pub fn d4(reader: BufReader<File>) -> Result<u32, String> {
    let mut result = 0;

    for line in reader.lines() {
        if let Ok(curr) = line {
            if let Some(new_val) = get_line_val(&curr) {
                result += new_val;
            }
        }
    }

    println!("RESULT: {}", result);

    Ok(result)
}


fn get_line_val(line: &str) -> Option<u32> {

    if let Some(counter) = count_winners(line) {
        if counter == 0 {
            return Some(0);
        } 
    
        let rez: u32 = 2_u32.pow(counter - 1);

        return Some(rez);
    }

    None
}