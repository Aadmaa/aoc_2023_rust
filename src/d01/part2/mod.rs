use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

 
const NUMBER_MAP: &'static [(&'static str, u32)] =
    &[
        ("one", 1), 
        ("two", 2), 
        ("three", 3), 
        ("four", 4), 
        ("five", 5), 
        ("six", 6), 
        ("seven", 7), 
        ("eight", 8), 
        ("nine", 9), 
    ];

// Returns, like, "one|two|three ..."
fn joined_number_strings() -> String {
    NUMBER_MAP.iter()
        .map(|(s, _)| *s)
        .collect::<Vec<_>>()
        .join("|")
}

// Returns, like, "eno|owt|eerht ..." - using the reverse of each number
fn joined_reversed_number_strings() -> String {
    NUMBER_MAP.iter()
        .map(|(s, _)| (*s).chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("|")
}

fn replace_first(line: &str) -> Result<String, String> {
    let pattern = format!(r"{}", joined_number_strings());
    let rex = Regex::new(&pattern).unwrap();
    let found = rex.find(line);
    let result = match found {
        Some(match_val) => {
            let match_string = &line[match_val.start()..match_val.end()];

            let &(_, num) = NUMBER_MAP.iter()
                .find(|(w, _)| *w == match_string).unwrap();

            let before = &line[..match_val.start()];
            let after = &line[match_val.end()..];
            format!("{}{}{}", before, num.to_string(), after) // Replace the match with "W"
        },
        None => line.to_string(),
    };

    Ok(result)
}

fn replace_last(line: &str) -> Result<String, String> {
    let line_rev = line.chars().rev().collect::<String>();
    let pattern = format!(r"{}", joined_reversed_number_strings());
    let rex = Regex::new(&pattern).unwrap();
    let found = rex.find(&line_rev);
    let result = match found {
        Some(match_val) => {
            let match_string = &line_rev[match_val.start()..match_val.end()];
            let match_string_rev = match_string.chars().rev().collect::<String>();

            let &(_, num) = NUMBER_MAP.iter()
                .find(|(w, _)| *w == match_string_rev).unwrap();

            let before = &line_rev[..match_val.start()];
            let after = &line_rev[match_val.end()..];
            let new_line_rev = format!("{}{}{}", before, num.to_string(), after);
            // Reverse the reversed string to get it in order again
            new_line_rev.chars().rev().collect::<String>()
        },
        None => line.to_string(),
    };

    Ok(result)
}


/// Example:  
/// 
/// ```rust,ignore
/// cargo run -- -p d1 -t b -f "./data/d01/sample_b.txt"
/// ````
pub fn d1_b(reader: BufReader<File>) -> Result<u32, String> {

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

    let line_left = replace_first(line)?;
    let line_right = replace_last(line)?;

    // let line = replace_outer_string_numbers(line)?;

    let rezz_left = rex.replace_all(&line_left, "".to_string());
    let rezz_right = rex.replace_all(&line_right, "".to_string());

    let first = rezz_left.chars().nth(0).unwrap();
    let last = rezz_right.chars().last().unwrap();

    let str = format!("{}{}", first, last);

    str.parse::<u32>().map_err(|e| e.to_string())
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_first() {
        let result = replace_first("abtwoc3fourdfive");
        assert_eq!(result, Ok("ab2c3fourdfive".to_string()));   
    }

    #[test]
    fn test_replace_last() {
        let result = replace_last("abtwoc3fourdfive");
        assert_eq!(result, Ok("abtwoc3fourd5".to_string()));   
    }

    #[test]
    fn test_get_line_val_samples() {
        assert_eq!(get_line_val("two1nine"), Ok(29));
        assert_eq!(get_line_val("eightwothree"), Ok(83));
        assert_eq!(get_line_val("abcone2threexyz"), Ok(13));
        assert_eq!(get_line_val("xtwone3four"), Ok(24));
        assert_eq!(get_line_val("4nineeightseven2"), Ok(42));
        assert_eq!(get_line_val("zoneight234"), Ok(14));
        assert_eq!(get_line_val("7pqrstsixteen"), Ok(76));
    }

    #[test]
    fn test_get_line_val_main() {
        assert_eq!(get_line_val("9eightone"), Ok(91));
        assert_eq!(get_line_val("hczsqfour3nxm5seven4"), Ok(44));
        assert_eq!(get_line_val("9twopjqkghmbone"), Ok(91));
        assert_eq!(get_line_val("rhrfthv886vflthreeztvzs"), Ok(83));
        assert_eq!(get_line_val("tlbtwo62five"), Ok(25));
        assert_eq!(get_line_val("ninetwonine234nvtlzxzczx"), Ok(94));
        assert_eq!(get_line_val("28sevenseven"), Ok(27));

        assert_eq!(get_line_val("2sevensxszqdhjg2threexzjj3"), Ok(23));
        assert_eq!(get_line_val("2fvq"), Ok(22));
        assert_eq!(get_line_val("781dk97eight26"), Ok(76));
        assert_eq!(get_line_val("plfrsjtbl6"), Ok(66));
        assert_eq!(get_line_val("sixglj13"), Ok(63));


        assert_eq!(get_line_val("sixglj13"), Ok(63));
        assert_eq!(get_line_val("b3seven6817gjpcxseven"), Ok(37));
        assert_eq!(get_line_val("3fivenlqcbszfoursixfive6sixfb"), Ok(36));
        assert_eq!(get_line_val("3three9"), Ok(39));
        assert_eq!(get_line_val("d6"), Ok(66));
        assert_eq!(get_line_val("eightwo"), Ok(82));

    }

}