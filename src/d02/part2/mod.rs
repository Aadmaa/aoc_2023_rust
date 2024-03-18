use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::d02::shared::{self, Subgame};

// use super::shared::Game;


/// Example:  
/// 
/// ```rust,ignore
/// cargo run -- -p d2 -t b -f "./data/d02/sample.txt"
/// cargo run -- -p d2 -t b -f "./data/d02/main.txt"
/// ````
pub fn d2_b(reader: BufReader<File>) -> Result<u32, String> {

    let mut result = 0;
    for line in reader.lines() {
        let line_val = line.map_err(|e| e.to_string())?;
        // println!("{}", line_val);
        result += get_line_val(&line_val);
    }

    print!("Result: {}\n", result);

    Ok(result)
}

fn get_line_val(line: &str) -> u32 {
    let game = match shared::line_to_game(line) {
        Some(val) => val,
        None => return 0
    };

    let mut acc = Subgame::new(0, 0, 0);

    let _ = game.subgames.iter().for_each(|subgame| {
        acc.b = max(acc.b, subgame.b);
        acc.r = max(acc.r, subgame.r);
        acc.g = max(acc.g, subgame.g);
    });

    let cubed = acc.b * acc.g * acc.r;

    cubed
}