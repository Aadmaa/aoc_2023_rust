use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::d02::shared;

use super::shared::Game;

const MAX_R: u32 = 12;
const MAX_G: u32 = 13;
const MAX_B: u32 = 14;

// 12 red cubes, 13 green cubes, and 14 blue

/// Example:  
/// 
/// ```rust,ignore
/// cargo run -- -p d2 -t a -f "./data/d02/sample.txt"
/// cargo run -- -p d2 -t a -f "./data/d02/main.txt"
/// ````
pub fn d2(reader: BufReader<File>) -> Result<u32, String> {

    let mut result = 0;
    for line in reader.lines() {
        let line_val = line.map_err(|e| e.to_string())?;
        // println!("{}", line_val);
        result += get_line_val(&line_val);
    }

    Ok(result)
}

fn game_val(g: &Game) -> u32 {
    let bad_game = g.subgames.iter()
        .any(|subgame|  subgame.b > MAX_B || subgame.r > MAX_R || subgame.g > MAX_G);

    // print!("id: {}  bad_game? {:?}\n", g.id, bad_game);

    match bad_game {
        false => g.id,
        true => 0,
    }
}

fn get_line_val(line: &str) -> u32 {
    let game = match shared::line_to_game(line) {
        Some(val) => val,
        None => return 0
    };

    // println!("{}", line);
    game_val(&game)
}